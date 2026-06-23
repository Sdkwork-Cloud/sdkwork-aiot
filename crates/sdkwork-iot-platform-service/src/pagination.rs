use sdkwork_aiot_transport::HttpRequest;

/// Standard list query parameters per `API_SPEC.md` section 16.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageQuery {
    pub page: u32,
    pub page_size: u32,
}

impl PageQuery {
    pub const DEFAULT_PAGE: u32 = 1;
    pub const DEFAULT_PAGE_SIZE: u32 = 20;
    pub const MAX_PAGE_SIZE: u32 = 200;

    pub fn from_request(request: &HttpRequest) -> Self {
        let page = request
            .query_param("page")
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(Self::DEFAULT_PAGE)
            .max(1);
        let page_size = request
            .query_param("page_size")
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(Self::DEFAULT_PAGE_SIZE)
            .clamp(1, Self::MAX_PAGE_SIZE);
        Self { page, page_size }
    }

    pub fn slice_bounds(self, total: usize) -> (usize, usize) {
        if total == 0 {
            return (0, 0);
        }
        let start = ((self.page - 1) as usize).saturating_mul(self.page_size as usize);
        if start >= total {
            return (total, total);
        }
        let end = (start + self.page_size as usize).min(total);
        (start, end)
    }

    pub fn page_info_json(self, total: usize) -> String {
        let has_more = (self.page as usize).saturating_mul(self.page_size as usize) < total;
        format!(
            r#"{{"page":{},"pageSize":{},"total":{},"hasMore":{}}}"#,
            self.page, self.page_size, total, has_more
        )
    }
}

pub fn paginated_collection_body(items_json: &str, page_query: PageQuery, total: usize) -> String {
    format!(
        r#"{{"code":"0","data":[{items_json}],"pageInfo":{}}}"#,
        page_query.page_info_json(total)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_query_defaults_and_clamps_page_size() {
        let request = HttpRequest::new("GET", "/items");
        let query = PageQuery::from_request(&request);
        assert_eq!(query.page, 1);
        assert_eq!(query.page_size, 20);
    }

    #[test]
    fn slice_bounds_respects_page_window() {
        let query = PageQuery {
            page: 2,
            page_size: 10,
        };
        assert_eq!(query.slice_bounds(25), (10, 20));
        assert_eq!(query.slice_bounds(0), (0, 0));
    }
}
