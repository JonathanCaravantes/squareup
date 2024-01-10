//! Query parameters for the Retrieve Catalog Object API

use std::fmt::Display;

/// This is a model struct for RetrieveCatalogObjectParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct RetrieveCatalogObjectParameters {
    /// If `true`, the response will include additional objects that are related to the requested
    /// objects. Related objects are defined as any objects referenced by ID by the results in the
    /// `objects` field of the response. These objects are put in the `related_objects` field.
    /// Setting this to `true` is helpful when the objects are needed for immediate display to a
    /// user. This process only goes one level deep. Objects referenced by the related objects will
    /// not be included. For example,
    ///
    /// if the `objects` field of the response contains a CatalogItem, its associated
    /// CatalogCategory objects, CatalogTax objects, CatalogImage objects and CatalogModifierLists
    /// will be returned in the `related_objects` field of the response. If the `objects` field of
    /// the response contains a CatalogItemVariation, its parent CatalogItem will be returned in the
    /// `related_objects` field of the response.
    ///
    /// Default value: `false`
    pub include_related_objects: Option<bool>,
    /// Requests objects as of a specific version of the catalog. This allows you to retrieve
    /// historical versions of objects. The value to retrieve a specific version of an object can be
    /// found in the version field of [CatalogObject]s. If not included, results will be from the
    /// current version of the catalog.
    pub catalog_version: Option<i64>,
    /// Specifies whether or not to include the path_to_root list for each returned category instance.
    /// The path_to_root list consists of CategoryPathToRootNode objects and specifies the path that
    /// starts with the immediate parent category of the returned category and ends with its root
    /// category. If the returned category is a top-level category, the path_to_root list is empty
    /// and is not returned in the response payload.
    pub include_category_path_to_root: Option<bool>,
}

impl RetrieveCatalogObjectParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<RetrieveCatalogObjectParameters> for String {
    fn from(retrieve_catalog_object_parameters: RetrieveCatalogObjectParameters) -> Self {
        retrieve_catalog_object_parameters.to_string()
    }
}

impl Display for RetrieveCatalogObjectParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(include_related_objects) = &self.include_related_objects {
            params.push(format!(
                "include_related_objects={}",
                include_related_objects
            ));
        }

        if let Some(catalog_version) = &self.catalog_version {
            params.push(format!("catalog_version={}", catalog_version));
        }

        if let Some(include_category_path_to_root) = &self.include_category_path_to_root {
            params.push(format!(
                "include_category_path_to_root={}",
                include_category_path_to_root
            ));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
