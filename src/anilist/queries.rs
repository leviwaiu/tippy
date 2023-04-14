pub const VIEWER_QUERY_STRING: &str = "
        query{
            Viewer{
                id
            }
        }";

pub const ANIME_LIST_PAGE:&str = "
        query($userId: Int, $page: Int, $perPage: Int){
            Page(page:$page, perPage: $perPage){
                pageInfo {
                    total
                    currentPage
                    lastPage
                    hasNextPage
                    perPage
                }
                mediaList(userId:$userId, type:ANIME){
                    id
                    media {
                        id
                        title {
                            romaji
                            native
                        }
                        episodes
                    }
                    score
                    progress
                    status
                }
            }
        }";

pub const ANIME_LIST_PAGE_FILTERED_STRING:&str = "
        query($userId: Int, $page: Int, $perPage: Int, $status: [MediaListStatus]){
            Page(page:$page, perPage: $perPage){
                pageInfo {
                    total
                    currentPage
                    lastPage
                    hasNextPage
                    perPage
                }
                mediaList(userId:$userId, type:ANIME, status_in: $status){
                    id
                    media {
                        id
                        title {
                            romaji
                            native
                        }
                        episodes
                    }
                    score
                    progress
                    status
                }
            }
        }";

pub const SEARCH_STRING:&str = "
        query($keyword: String, $page: Int, $perPage: Int){
            Page(page:$page, perPage: $perPage){
                pageInfo {
                    total
                    currentPage
                    lastPage
                    hasNextPage
                    perPage
                }
                media(type:ANIME, search:$keyword){
                    id
                    title {
                        romaji
                        native
                    }
                    format
                    season
                    seasonYear
                }
            }
        }
        ";

pub const EDIT_WATCHCOUNT_STRING:&str = "
        mutation($id: Int, $progress: Int){
            SaveMediaListEntry(id: $id, progress:$progress) {
                id
                progress
            }
        }
        ";

pub const ANIME_DETAIL_QUERY_STRING:&str = "
        query($media_id: Int){
            Media(id:$media_id){
                id
                type
                studios(isMain:true) {
                    nodes {
                        id
                        name
                    }
                }
                episodes
                duration
                startDate {
                    year
                    month
                    day
                }
                endDate {
                    year
                    month
                    day
                }
            }
        }
";