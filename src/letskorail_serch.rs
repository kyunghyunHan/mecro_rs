use std::error::Error;
use std::time::Duration;
use thirtyfour::{common, prelude::*};
use tokio::time::sleep;
pub async fn example() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Chrome 브라우저 옵션 정의
    let mut caps = DesiredCapabilities::chrome();
    // Headless 모드로 Chrome 실행
    // caps.add_arg("--headless=new")?; // 개발 중에는 주석 처리 가능
    // 지정한 옵션으로 Chrome 인스턴스를 제어하는 드라이버 초기화
    let driver = WebDriver::new("http://localhost:56095", caps).await?;

    // Google 홈페이지 방문
    sleep(Duration::from_secs(1)).await;

    driver
        .goto("https://www.letskorail.com/ebizprd/EbizPrdTicketpr21100W_pr21110.do")
        .await?;

    // // name 속성으로 검색 상자를 찾음
    let search_start_box = driver.find_element(By::Name("txtGoStart")).await?;
    let search_end_box = driver.find_element(By::Name("txtGoEnd")).await?;
    let search_btn = driver.find_element(By::ClassName("btn_inq")).await?;

    search_start_box.clear().await?;
    search_start_box.send_keys("전주").await?;

    search_end_box.clear().await?;
    search_end_box.send_keys("서울").await?;
    // let year_select = driver.find_element(By::Id("s_year")).await?;
    // year_select.click().await?;
    driver
        .execute("document.getElementById('s_year').value = '2024';", vec![])
        .await?;
    driver
        .execute(
            "document.getElementById('s_year').dispatchEvent(new Event('change'));",
            vec![],
        )
        .await?;
    driver
        .execute("document.getElementById('s_month').value = '10';", vec![])
        .await?;
    driver
        .execute(
            "document.getElementById('s_month').dispatchEvent(new Event('change'));",
            vec![],
        )
        .await?;
    driver
        .execute("document.getElementById('s_day').value = '20';", vec![])
        .await?;
    driver
        .execute(
            "document.getElementById('s_day').dispatchEvent(new Event('change'));",
            vec![],
        )
        .await?;
    driver
        .execute("document.getElementById('s_hour').value = '15';", vec![])
        .await?;
    driver
        .execute(
            "document.getElementById('s_hour').dispatchEvent(new Event('change'));",
            vec![],
        )
        .await?;
    sleep(Duration::from_secs(3)).await;
    search_btn.click().await?;

    sleep(Duration::from_secs(3)).await;
    //  let td_elements = driver.find_elements(By::XPath(r#"//td[contains(normalize-space(), '전주')]"#)).await?;
    let td_elements = driver
        .find_elements(By::XPath(
            r#"//td[contains(normalize-space(), '전주') and contains(normalize-space(), '16:25')]"#,
        ))
        .await;
    // 각 <td> 요소의 텍스트 가져오기
    match td_elements {
        Ok(tds) if !tds.is_empty() => {
            for td in tds {
                // 부모 <tr> 요소 찾기
                let row = td.find_element(By::XPath("ancestor::tr")).await?;

                // 같은 행의 모든 <td> 요소 찾기
                let all_td_elements = row.find_elements(By::XPath("./td")).await?;

                // 모든 <td> 요소의 텍스트 출력
                // for cell in all_td_elements {
                //     let cell_text = cell.text().await;
                //     println!("Cell text: {:?}", cell_text);
                // }
                for cell in all_td_elements {
                    println!("{:?}", cell.text().await);
                    // 버튼 요소를 찾기
                    let reserve_button = cell
                        .find_elements(By::XPath(".//a[img[contains(@alt,'입좌석묶음예약')]]"))
                        .await?;
                    println!("{}", reserve_button.len());
                    // 버튼이 발견된 경우 클릭
                    if let Some(button) = reserve_button.get(0) {
                        button.click().await?;
                        driver.switch_to().alert().accept().await.unwrap();
                    } else {
                        println!("No buttons found in this cell.");
                    }
                }
            }
            // break; // 전주와 16:25이 포함된 <td>를 찾았으므로 루프 종료
        }
        _ => {
            println!("No matching <td> elements found. Retrying...");
        }
    }
    sleep(Duration::from_secs(1)).await;

    driver.quit().await?;

    Ok(())
}
