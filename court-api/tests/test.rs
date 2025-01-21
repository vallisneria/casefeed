#[cfg(test)]
mod test {
    use court_api::constitutional_court::{
        BenchType, ConstitutionCaseType, ConstitutionalCaseSearchParameter,
    };

    #[tokio::test]
    async fn example_search() {
        let search = ConstitutionalCaseSearchParameter::default()
            // 전원재판부
            .set_bench_type(vec![BenchType::EnBancBench])
            // 정당해산, 탄핵
            .set_case_type(vec![
                ConstitutionCaseType::DissolutionParty,
                ConstitutionCaseType::Impeachment,
            ]);

        let response = search.search().await.unwrap();
        println!("{response:#?}")
    }

    #[test]
    fn test() {
        use court_api::constitutional_court::ConstitutionDecisionType;
        use serde_json::from_str;

        let item: ConstitutionDecisionType = from_str(r#""기각""#).unwrap();
        println!("{item:?}")
    }
}
