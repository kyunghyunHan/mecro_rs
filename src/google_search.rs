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
    driver.goto("https://www.google.com").await?;

    // name 속성으로 검색 상자를 찾음
    let search_box = driver.find_element(By::Name("q")).await?;

    // 입력 상자를 초기화한 후 "a" 입력
    search_box.clear().await?;
    search_box.send_keys("a").await?;
    search_box.send_keys(common::keys::Key::Enter).await?;

    // 검색어가 입력된 후 약간의 대기 시간 추가
    sleep(Duration::from_secs(2)).await;

    // 결과 로딩 대기
    driver.find_element(By::Css("div#search")).await?;

    // 결과 페이지의 HTML 소스를 가져옴
    let html = driver.source().await?;
    println!("{html}");

    // 브라우저 닫기
    driver.quit().await?;

    Ok(())
}
