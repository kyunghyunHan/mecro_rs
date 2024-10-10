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
    let year_select = driver.find_element(By::Id("s_year")).await?;
    year_select.click().await?;

    let option_2024 = driver
        .find_element(By::XPath(r#".//option[@value='2024']"#))
        .await?;
    option_2024.click().await?;

    sleep(Duration::from_secs(1)).await;

    let month_select = driver.find_element(By::Id("s_month")).await?;
    month_select.click().await?;
    let option_month = driver
        .find_element(By::XPath(r#".//option[@value='10']"#))
        .await?;
    option_month.click().await?;

    let day_select = driver.find_element(By::Id("s_day")).await?;
    day_select.click().await?;

    let option_day = driver
        .find_element(By::XPath(r#".//option[@value='20']"#))
        .await?;
    option_day.click().await?;
    sleep(Duration::from_secs(1)).await;

    let hour_select = driver.find_element(By::Id("s_hour")).await?;
    hour_select.click().await?;
    sleep(Duration::from_secs(1)).await;

    let option_hour = driver
        .find_element(By::XPath(r#".//option[@value='15']"#))
        .await?;

    option_hour.click().await?;

    sleep(Duration::from_secs(1)).await;

    search_btn.click().await?;

    // // 검색어가 입력된 후 약간의 대기 시간 추가
    // sleep(Duration::from_secs(2)).await;

    // // 결과 로딩 대기
    // driver.find_element(By::Css("div#search")).await?;

    // // 결과 페이지의 HTML 소스를 가져옴
    // let html = driver.source().await?;
    // println!("{html}");

    // 브라우저 닫기
    sleep(Duration::from_secs(20)).await;

    driver.quit().await?;

    Ok(())
}
