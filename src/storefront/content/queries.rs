use std::sync::OnceLock;

use crate::storefront::fragments::{IMAGE_FRAGMENT, SEO_FRAGMENT};

const PAGE_FRAGMENT: &str = r#"
  fragment PageFields on Page {
    id
    handle
    title
    body
    bodySummary
    createdAt
    updatedAt
    onlineStoreUrl
    seo {
      ...SEOFields
    }
  }
"#;

const BLOG_FRAGMENT: &str = r#"
  fragment BlogFields on Blog {
    id
    handle
    title
    onlineStoreUrl
    seo {
      ...SEOFields
    }
  }
"#;

const ARTICLE_FRAGMENT: &str = r#"
  fragment ArticleFields on Article {
    id
    handle
    title
    content
    contentHtml
    excerpt
    excerptHtml
    publishedAt
    tags
    onlineStoreUrl
    image {
      ...ImageFields
    }
    seo {
      ...SEOFields
    }
    authorV2 {
      name
      email
      bio
    }
    blog {
      id
      handle
      title
    }
  }
"#;

const MENU_ITEM_FRAGMENT: &str = r#"
  fragment MenuItemFields on MenuItem {
    id
    title
    type
    url
    items {
      id
      title
      type
      url
      items {
        id
        title
        type
        url
      }
    }
  }
"#;

const MENU_FRAGMENT: &str = r#"
  fragment MenuFields on Menu {
    id
    handle
    title
    items {
      ...MenuItemFields
    }
  }
"#;

const GET_PAGE_BY_ID: &str = r#"
  query GetPageById($id: ID!) {
    page(id: $id) {
      ...PageFields
    }
  }
"#;

const GET_PAGE_BY_HANDLE: &str = r#"
  query GetPageByHandle($handle: String!) {
    page(handle: $handle) {
      ...PageFields
    }
  }
"#;

const GET_PAGES: &str = r#"
  query GetPages(
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: PageSortKeys
    $query: String
  ) {
    pages(
      first: $first
      after: $after
      last: $last
      before: $before
      reverse: $reverse
      sortKey: $sortKey
      query: $query
    ) {
      nodes {
        ...PageFields
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

const GET_BLOG_BY_ID: &str = r#"
  query GetBlogById($id: ID!) {
    blog(id: $id) {
      ...BlogFields
    }
  }
"#;

const GET_BLOG_BY_HANDLE: &str = r#"
  query GetBlogByHandle($handle: String!) {
    blog(handle: $handle) {
      ...BlogFields
    }
  }
"#;

const GET_BLOGS: &str = r#"
  query GetBlogs(
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: BlogSortKeys
    $query: String
  ) {
    blogs(
      first: $first
      after: $after
      last: $last
      before: $before
      reverse: $reverse
      sortKey: $sortKey
      query: $query
    ) {
      nodes {
        ...BlogFields
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

const GET_BLOG_WITH_ARTICLES: &str = r#"
  query GetBlogWithArticles(
    $id: ID
    $handle: String
    $articlesFirst: Int
    $articlesAfter: String
    $articlesReverse: Boolean
    $articlesSortKey: ArticleSortKeys
  ) {
    blog(id: $id, handle: $handle) {
      ...BlogFields
      articles(
        first: $articlesFirst
        after: $articlesAfter
        reverse: $articlesReverse
        sortKey: $articlesSortKey
      ) {
        nodes {
          ...ArticleFields
        }
        pageInfo {
          hasNextPage
          hasPreviousPage
          startCursor
          endCursor
        }
      }
    }
  }
"#;

const GET_ARTICLE_BY_ID: &str = r#"
  query GetArticleById($id: ID!) {
    article(id: $id) {
      ...ArticleFields
    }
  }
"#;

const GET_ARTICLES: &str = r#"
  query GetArticles(
    $first: Int
    $after: String
    $last: Int
    $before: String
    $reverse: Boolean
    $sortKey: ArticleSortKeys
    $query: String
  ) {
    articles(
      first: $first
      after: $after
      last: $last
      before: $before
      reverse: $reverse
      sortKey: $sortKey
      query: $query
    ) {
      nodes {
        ...ArticleFields
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

const GET_MENU: &str = r#"
  query GetMenu($handle: String!) {
    menu(handle: $handle) {
      ...MenuFields
    }
  }
"#;

pub fn get_page_by_id() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [SEO_FRAGMENT, PAGE_FRAGMENT, GET_PAGE_BY_ID].concat())
}

pub fn get_page_by_handle() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [SEO_FRAGMENT, PAGE_FRAGMENT, GET_PAGE_BY_HANDLE].concat())
}

pub fn get_pages() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [SEO_FRAGMENT, PAGE_FRAGMENT, GET_PAGES].concat())
}

pub fn get_blog_by_id() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [SEO_FRAGMENT, BLOG_FRAGMENT, GET_BLOG_BY_ID].concat())
}

pub fn get_blog_by_handle() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [SEO_FRAGMENT, BLOG_FRAGMENT, GET_BLOG_BY_HANDLE].concat())
}

pub fn get_blogs() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [SEO_FRAGMENT, BLOG_FRAGMENT, GET_BLOGS].concat())
}

pub fn get_blog_with_articles() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            IMAGE_FRAGMENT,
            SEO_FRAGMENT,
            BLOG_FRAGMENT,
            ARTICLE_FRAGMENT,
            GET_BLOG_WITH_ARTICLES,
        ]
        .concat()
    })
}

pub fn get_article_by_id() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| {
        [
            IMAGE_FRAGMENT,
            SEO_FRAGMENT,
            ARTICLE_FRAGMENT,
            GET_ARTICLE_BY_ID,
        ]
        .concat()
    })
}

pub fn get_articles() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [IMAGE_FRAGMENT, SEO_FRAGMENT, ARTICLE_FRAGMENT, GET_ARTICLES].concat())
}

pub fn get_menu() -> &'static str {
    static Q: OnceLock<String> = OnceLock::new();
    Q.get_or_init(|| [MENU_ITEM_FRAGMENT, MENU_FRAGMENT, GET_MENU].concat())
}
