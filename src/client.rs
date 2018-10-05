use failure::Error;
use feed::Feed;
use package::Package;
use reqwest::Body;
use reqwest::{Client as ReqwestClient, Response};
use serde::Deserialize;
use serde_xml_rs;
use url::Url;

pub struct Client {
    client: ReqwestClient,
    pub base_url: Url,
    api_key: Option<String>,
}

impl Client {
    pub fn new(url: Url, api_key: Option<String>) -> Result<Client, Error> {
        let mut base_url: Url = url.clone();
        // URI joiners will replace the entire path if the base URI
        // doesn't end in '/'
        base_url
            .path_segments_mut()
            .unwrap()
            .pop_if_empty()
            .push("");

        Ok(Client {
            client: ReqwestClient::new(),
            base_url: base_url,
            api_key: api_key,
        })
    }

    pub fn packages(&self, filter: Option<&str>) -> Result<Vec<Package>, Error> {
        let mut results = Vec::new();

        let page_size = 15_000u64;
        let mut skip = 0u64;

        loop {
            let mut url = self.base_url.join("Packages()")?;

            {
                let mut query_pairs = url.query_pairs_mut();

                query_pairs
                    .clear()
                    .append_pair("$top", &page_size.to_string())
                    .append_pair("$skip", &skip.to_string());

                if let Some(filter) = filter {
                    query_pairs.append_pair("$filter", filter);
                }
            }

            debug!("GET {}", &url);

            let mut feed: Feed = self.get_xml(&url)?;

            if feed.packages.is_empty() {
                break;
            } else {
                results.append(&mut feed.packages);
                skip += page_size;
            }
        }

        Ok(results)
    }

    pub fn package(&self, id: &str, version: &str) -> Result<Package, Error> {
        let url = self
            .base_url
            .join(&format!("Packages(Id='{}',Version='{}')", id, version))?;

        self.get_xml(&url)
    }

    pub fn package_versions(&self, id: &str) -> Result<Vec<Package>, Error> {
        let mut url = self.base_url.join(&format!("FindPackagesById()"))?;
        url.query_pairs_mut().append_pair("id", id);

        self.get_xml(&url)
    }

    pub fn search(
        &self,
        search_term: &str,
        target_framework: &str,
        include_prerelease: bool,
    ) -> Result<Vec<Package>, Error> {
        let mut url = self.base_url.join(&format!("Search()"))?;
        url.query_pairs_mut()
            .append_pair("searchTerm", search_term)
            .append_pair("targetFramework", target_framework)
            .append_pair("includePrerelease", &include_prerelease.to_string());

        self.get_xml(&url)
    }

    pub fn delete_package(&self, id: &str, version: &str) -> Result<Response, Error> {
        let url = self.base_url.join(id)?.join(version)?;

        debug!("DELETE {}", &url);

        self.client
            .delete(url.as_str())
            .header("X-NuGet-ApiKey", self.api_key.clone().unwrap())
            .send()?
            .error_for_status()
            .map_err(|x| x.into())
    }

    pub fn push_package<T>(&self, package_content: T) -> Result<Response, Error>
    where
        T: Into<Body>,
    {
        debug!("Pushing Package");

        self.client
            .post(self.base_url.as_str())
            .header("X-NuGet-ApiKey", self.api_key.clone().unwrap())
            .body(package_content)
            .send()?
            .error_for_status()
            .map_err(|x| x.into())
    }

    pub fn get_xml<'de, T>(&self, url: &Url) -> Result<T, Error>
    where
        T: Deserialize<'de>,
    {
        self.get(url).and_then(|res| {
            serde_xml_rs::from_reader::<_, T>(res)
                .map_err(|_| format_err!("Unable to deserialize {}", url))
        })
    }

    pub fn get(&self, url: &Url) -> Result<Response, Error> {
        debug!("GET {}", url);

        self.client
            .get(url.as_str())
            .send()?
            .error_for_status()
            .map_err(|e| e.into())
    }

    /*pub fn put_file(&self, path: &str, body: &[u8]) -> Result<Response, Error> {
        let url = self.base_url.join(path)?;
        let api_key = self.api_key.clone().unwrap();

        let mut body_headers = hyper::header::Headers::new();
        body_headers.set(hyper::header::ContentType(
            "application/octet-stream"
                .parse::<hyper::mime::Mime>()
                .map_err(|_| format_err!("Failed to parse mime type application/octet-stream"))?,
        ));

        let tmpdir = TempDir::new("nuget-rs")?;
        let tmppath = tmpdir.path().join("package.nupkg");
        let mut tmpfile = File::create(tmppath.clone())?;
        let _ = tmpfile.write_all(body);

        let mut buffer = vec![];
        let formdata = FormData {
            fields: vec![],
            files: vec![(
                "package".to_owned(),
                FilePart::new(body_headers, Path::new(&tmppath)),
            )],
        };

        let boundary = formdata::generate_boundary();
        let _ = formdata::write_formdata_chunked(&mut buffer, &boundary, &formdata);

        debug!("PUT {}", url);

        let req = self
            .client
            .put(url.as_str())
            .header("X-NuGet-ApiKey", api_key)
            .header(TRANSFER_ENCODING, "chunked")
            .header(
                CONTENT_TYPE,
                format!(
                    "multipart/form-data; boundary=\"{}\"",
                    String::from_utf8(boundary)?
                ),
            ).header(CONTENT_LENGTH, buffer.len() as u64)
            .body(buffer);

        let resp = req.send()?;

        debug!("Resp headers: {:?}", &resp.headers());

        resp.error_for_status().map_err(|x| x.into())
    }*/
}
