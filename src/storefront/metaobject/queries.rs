use std::sync::OnceLock;

const METAOBJECT_FIELD_FRAGMENT: &str = r#"
  fragment MetaobjectFieldFields on MetaobjectField {
    key
    value
    type
    reference {
      __typename
      ... on Product {
        id
        handle
        title
      }
      ... on Collection {
        id
        handle
        title
      }
      ... on Page {
        id
        handle
        title
      }
      ... on MediaImage {
        id
        image {
          url
          altText
          width
          height
        }
      }
      ... on Metaobject {
        id
        handle
        type
      }
    }
    references(first: 10) {
      nodes {
        __typename
        ... on Product {
          id
          handle
          title
        }
        ... on Collection {
          id
          handle
          title
        }
        ... on Page {
          id
          handle
          title
        }
        ... on MediaImage {
          id
          image {
            url
            altText
            width
            height
          }
        }
        ... on Metaobject {
          id
          handle
          type
        }
      }
    }
  }
"#;

const METAOBJECT_FRAGMENT: &str = r#"
  fragment MetaobjectFields on Metaobject {
    id
    handle
    type
    updatedAt
    fields {
      ...MetaobjectFieldFields
    }
  }
"#;

const GET_METAOBJECT_BY_ID: &str = r#"
  query GetMetaobjectById($id: ID!) {
    metaobject(id: $id) {
      ...MetaobjectFields
    }
  }
"#;

const GET_METAOBJECT_BY_HANDLE: &str = r#"
  query GetMetaobjectByHandle($handle: MetaobjectHandleInput!) {
    metaobject(handle: $handle) {
      ...MetaobjectFields
    }
  }
"#;

const GET_METAOBJECTS: &str = r#"
  query GetMetaobjects(
    $type: String!
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: String
  ) {
    metaobjects(
      type: $type
      first: $first
      after: $after
      last: $last
      before: $before
      reverse: $reverse
      sortKey: $sortKey
    ) {
      nodes {
        ...MetaobjectFields
      }
      pageInfo {
        hasNextPage
        hasPreviousPage
        startCursor
        endCursor
      }
    }
  }
"#;

pub fn get_metaobject_by_id() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            METAOBJECT_FIELD_FRAGMENT,
            METAOBJECT_FRAGMENT,
            GET_METAOBJECT_BY_ID,
        ]
        .concat()
    })
}

pub fn get_metaobject_by_handle() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            METAOBJECT_FIELD_FRAGMENT,
            METAOBJECT_FRAGMENT,
            GET_METAOBJECT_BY_HANDLE,
        ]
        .concat()
    })
}

pub fn get_metaobjects() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            METAOBJECT_FIELD_FRAGMENT,
            METAOBJECT_FRAGMENT,
            GET_METAOBJECTS,
        ]
        .concat()
    })
}
