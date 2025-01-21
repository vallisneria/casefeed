use super::{
    BenchType, ConstitutionCaseType, ConstitutionDecisionType, ConstitutionalCaseSearchParameter,
    RecordType, Sort,
};

impl ConstitutionalCaseSearchParameter {
    pub fn set_page(mut self, value: u32) -> Self {
        self.page = value;
        self
    }

    pub fn set_keyword<IntoString>(mut self, value: IntoString) -> Self
    where
        IntoString: Into<String>,
    {
        self.keyword = value.into();
        self
    }

    pub fn set_from<IntoString>(mut self, value: IntoString) -> Self
    where
        IntoString: Into<String>,
    {
        self.from = value.into();
        self
    }

    pub fn set_to<IntoString>(mut self, value: IntoString) -> Self
    where
        IntoString: Into<String>,
    {
        self.to = value.into();
        self
    }

    pub fn set_limit<IntoU8: Into<u8>>(mut self, value: IntoU8) -> Self {
        self.limit = value.into();
        self
    }

    pub fn set_sort<IntoSort>(mut self, value: IntoSort) -> Self
    where
        IntoSort: Into<Sort>,
    {
        self.sort = value.into();
        self
    }

    pub fn set_case_code<IntoString>(mut self, value: IntoString) -> Self
    where
        IntoString: Into<String>,
    {
        self.case_code = Some(value.into());
        self
    }

    pub fn set_case_code_option<IntoString>(mut self, value: Option<IntoString>) -> Self
    where
        IntoString: Into<String>,
    {
        self.case_code = value.map(|item| item.into());
        self
    }

    pub fn set_case_name<IntoString>(mut self, value: IntoString) -> Self
    where
        IntoString: Into<String>,
    {
        self.case_name = Some(value.into());
        self
    }

    pub fn set_case_name_option<IntoString>(mut self, value: Option<IntoString>) -> Self
    where
        IntoString: Into<String>,
    {
        self.case_name = value.map(|item| item.into());
        self
    }

    pub fn set_case_type(mut self, value: Vec<ConstitutionCaseType>) -> Self {
        self.case_type = value;
        self
    }

    pub fn set_decision_type(mut self, value: Vec<ConstitutionDecisionType>) -> Self {
        self.decision_type = value;
        self
    }

    pub fn set_bench_type(mut self, value: Vec<BenchType>) -> Self {
        self.bench_type = value;
        self
    }

    pub fn set_record_type(mut self, value: Vec<RecordType>) -> Self {
        self.record_type = value;
        self
    }

    pub fn set_exclusion_keyword<IntoString>(mut self, value: Vec<IntoString>) -> Self
    where
        IntoString: Into<String>,
    {
        self.exclusion_keyword = value.into_iter().map(|item| item.into()).collect();
        self
    }
}
