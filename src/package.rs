use client::Client;
use failure::Error;
use serde::de::Error as DeserializeError;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::io::Read;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub id: String,
    pub category: PackageCategory,
    // Links are not available due to split vecs panicking.
    // #[serde(rename = "link")]
    // pub links: Vec<Link>,
    pub title: String,
    pub summary: Option<String>,
    pub content: PackageContent,
    #[serde(rename = "updated")]
    pub updated_at: String,
    pub author: Author,
    pub properties: PackageProperties,
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.version() == other.version()
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PackageCategory {
    pub term: String,
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct PackageContent {
    #[serde(rename = "type")]
    pub mime_type: String,
    #[serde(rename = "src")]
    pub url: Url,
}

// Struct fields based on https://joelverhagen.github.io/NuGetUndocs/#package-entity
#[serde(rename_all = "PascalCase")]
#[derive(Debug, Deserialize, PartialEq)]
pub struct PackageProperties {
    // Common fields
    copyright: String,
    #[serde(deserialize_with = "parse_dependencies")]
    dependencies: Vec<PackageDependency>,
    description: String,
    download_count: i32,
    is_absolute_latest_version: bool,
    is_latest_version: bool,
    is_prerelease: bool,
    package_size: u64,
    project_url: String, // Url
    release_notes: String,
    require_license_acceptance: bool,
    tags: String,
    title: String,
    version: String,

    // Vendor specific fields
    authors: Option<String>,
    #[serde(rename = "Created")]
    created_at: Option<String>, // Date
    development_dependency: Option<bool>,
    gallery_details_url: Option<String>, // Option<Url>
    icon_url: Option<String>,            // Url
    id: Option<String>,
    #[serde(rename = "LastEdited")]
    last_edited_at: Option<String>, // Option<Date>
    #[serde(rename = "LastUpdated")]
    last_updated_at: Option<String>, // Option<Date>
    license_url: Option<String>, // Url
    license_names: Option<String>,
    license_report_url: Option<String>, // Option<Url>
    #[serde(rename = "Listed")]
    is_listed: Option<bool>,
    language: Option<String>,
    #[serde(rename = "MinClientVersion")]
    minimum_client_version: Option<String>,
    normalized_version: Option<String>,
    owners: Option<String>,
    package_hash: Option<String>,
    package_hash_algorithm: Option<String>,
    #[serde(rename = "Published")]
    published_at: Option<String>, // Date
    report_abuse_url: Option<String>, // Option<Url>
    summary: Option<String>,          // String
    version_download_count: i32,

    // ProGet specific fields
    is_local_package: Option<bool>,
    #[serde(rename = "IsProGetHosted")]
    is_proget_hosted: Option<bool>,
    is_cached: Option<bool>,

    has_symbols: Option<bool>,
    has_source: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PackageDependency {
    pub package_id: String,
    pub version: String,
    pub framework: String,
}

impl FromStr for PackageDependency {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted = s.splitn(3, ":").collect::<Vec<_>>();

        match splitted.len() {
            2 => Ok(Self {
                package_id: String::from(splitted[0]),
                version: String::from(splitted[1]),
                framework: String::from(""),
            }),
            3 => Ok(Self {
                package_id: String::from(splitted[0]),
                version: String::from(splitted[1]),
                framework: String::from(splitted[2]),
            }),
            _ => bail!(
                "2 or 3 elements are required for a package dependency. {} only has {}",
                s,
                splitted.len()
            ),
        }
    }
}

fn parse_dependencies<'de, D>(deserializer: D) -> Result<Vec<PackageDependency>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .split("|")
        .filter(|&elem| elem != "")
        .map(|elem| elem.parse().map_err(DeserializeError::custom))
        .collect()
}

#[serde(deny_unknown_fields)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
}

impl Package {
    pub fn id(&self) -> &str {
        self.properties.id.as_ref().unwrap_or(&self.title)
    }

    pub fn version(&self) -> &str {
        &self.properties.version
    }

    pub fn content(&self, client: &Client) -> Result<Vec<u8>, Error> {
        let url = &self.content.url;
        let mut resp = client.get(url)?;

        let mut buffer = vec![];
        let _ = resp.read_to_end(&mut buffer);
        Ok(buffer)
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.title, self.properties.version)
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_xml_rs;

    use super::*;

    #[test]
    fn nuget_gallery_package() {
        // From https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')
        let package_serialized = r##"<?xml version="1.0" encoding="UTF-8"?>
<entry xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:georss="http://www.georss.org/georss" xmlns:gml="http://www.opengis.net/gml" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="https://www.nuget.org/api/v2">
   <id>https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')</id>
   <category term="NuGetGallery.OData.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
   <link rel="edit" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
   <link rel="self" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
   <title type="text">_51Wp.AccountSdk</title>
   <updated>2015-12-15T15:05:02Z</updated>
   <author>
      <name>authors</name>
   </author>
   <content type="application/zip" src="https://www.nuget.org/api/v2/package/_51Wp.AccountSdk/1.0.0" />
   <m:properties>
      <d:Id>_51Wp.AccountSdk</d:Id>
      <d:Version>1.0.0</d:Version>
      <d:NormalizedVersion>1.0.0</d:NormalizedVersion>
      <d:Authors>authors</d:Authors>
      <d:Copyright m:null="true" />
      <d:Created m:type="Edm.DateTime">2015-12-15T15:05:02.15Z</d:Created>
      <d:Dependencies />
      <d:Description>My package description.</d:Description>
      <d:DownloadCount m:type="Edm.Int32">2195</d:DownloadCount>
      <d:GalleryDetailsUrl>https://www.nuget.org/packages/_51Wp.AccountSdk/1.0.0</d:GalleryDetailsUrl>
      <d:IconUrl m:null="true" />
      <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
      <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">false</d:IsAbsoluteLatestVersion>
      <d:IsPrerelease m:type="Edm.Boolean">false</d:IsPrerelease>
      <d:Language m:null="true" />
      <d:LastUpdated m:type="Edm.DateTime">2015-12-15T15:05:02.15Z</d:LastUpdated>
      <d:Published m:type="Edm.DateTime">1900-01-01T00:00:00</d:Published>
      <d:PackageHash>CwkBmkdSDYieaAgZxyrFizngyNfBB76piK7KFe7T8WgRH7opJZLiz6LdO3CCHp0u0E2GVazgbzAPJG+PNpzT1g==</d:PackageHash>
      <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
      <d:PackageSize m:type="Edm.Int64">212213</d:PackageSize>
      <d:ProjectUrl m:null="true" />
      <d:ReportAbuseUrl>https://www.nuget.org/packages/_51Wp.AccountSdk/1.0.0/ReportAbuse</d:ReportAbuseUrl>
      <d:ReleaseNotes m:null="true" />
      <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
      <d:Summary />
      <d:Tags />
      <d:Title>_51Wp.AccountSdk</d:Title>
      <d:VersionDownloadCount m:type="Edm.Int32">2195</d:VersionDownloadCount>
      <d:MinClientVersion m:null="true" />
      <d:LastEdited m:type="Edm.DateTime">2015-12-15T22:58:39.043Z</d:LastEdited>
      <d:LicenseUrl m:null="true" />
      <d:LicenseNames m:null="true" />
      <d:LicenseReportUrl m:null="true" />
   </m:properties>
</entry>"##;

        let package: Package = serde_xml_rs::from_reader(package_serialized.as_bytes()).unwrap();

        assert_eq!(
            package,
            Package {
                id: String::from("https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')"),
                category: PackageCategory {
                    term: String::from("NuGetGallery.OData.V2FeedPackage"),
                },
                title: String::from("_51Wp.AccountSdk"),
                summary: None,
                updated_at: String::from("2015-12-15T15:05:02Z"),
                author: Author {
                    name: String::from("authors")
                },
                content: PackageContent {
                    mime_type: String::from("application/zip"),
                    url: Url::parse(
                        "https://www.nuget.org/api/v2/package/_51Wp.\
                         AccountSdk/1.0.0"
                    ).unwrap(),
                },
                properties: PackageProperties {
                    owners: None,
                    development_dependency: None,
                    id: Some(String::from("_51Wp.AccountSdk")),
                    version: String::from("1.0.0"),
                    normalized_version: Some(String::from("1.0.0")),
                    authors: Some(String::from("authors")),
                    copyright: String::from(""),
                    created_at: Some(String::from("2015-12-15T15:05:02.15Z")),
                    dependencies: vec![],
                    description: String::from("My package description."),
                    download_count: 2195,
                    gallery_details_url: Some(String::from(
                        "https://www.nuget.\
                         org/packages/_51Wp.\
                         AccountSdk/1.0.0"
                    )),
                    icon_url: Some(String::from("")),
                    is_latest_version: false,
                    is_absolute_latest_version: false,
                    is_prerelease: false,
                    language: Some(String::from("")),
                    last_updated_at: Some(String::from("2015-12-15T15:05:02.15Z")),
                    published_at: Some(String::from("1900-01-01T00:00:00")),
                    package_hash: Some(String::from("CwkBmkdSDYieaAgZxyrFizngyNfBB76piK7KFe7T8WgRH7opJZLiz6LdO3CCHp0u0E2GVazgbzAPJG+PNpzT1g==")),
                    package_hash_algorithm: Some(String::from("SHA512")),
                    package_size: 212213,
                    project_url: String::from(""),
                    report_abuse_url: Some(String::from(
                        "https://www.nuget.\
                         org/packages/_51Wp.AccountSdk/1.\
                         0.0/ReportAbuse"
                    )),
                    release_notes: String::from(""),
                    require_license_acceptance: false,
                    summary: Some(String::from("")),
                    tags: String::from(""),
                    title: String::from("_51Wp.AccountSdk"),
                    version_download_count: 2195,
                    minimum_client_version: Some(String::from("")),
                    last_edited_at: Some(String::from("2015-12-15T22:58:39.043Z")),
                    license_url: Some(String::from("")),
                    license_names: Some(String::from("")),
                    license_report_url: Some(String::from("")),

                    // ProGet specific fields
                    is_local_package: None,
                    is_proget_hosted: None,
                    is_cached: None,
                    is_listed: None,
                    has_symbols: None,
                    has_source: None,
                },
            }
        );
    }

    #[test]
    fn myget_package() {
        // From https://www.myget.org/F/omnisharp/api/v2/Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')
        let package_serialized =
r##"<?xml version="1.0" encoding="UTF-8"?>
<entry xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="https://www.myget.org/F/omnisharp/api/v2/">
   <id>https://www.myget.org/F/omnisharp/api/v2/Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')</id>
   <category term="MyGet.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
   <link rel="edit" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')" />
   <link rel="http://schemas.microsoft.com/ado/2007/08/dataservices/related/Screenshots" type="application/atom+xml;type=feed" title="Screenshots" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/Screenshots" />
   <title type="text">Microsoft.Extensions.Primitives</title>
   <summary type="text">ASP.NET 5 primitives.</summary>
   <updated>2016-01-22T20:46:59Z</updated>
   <author>
      <name>Microsoft.Extensions.Primitives</name>
   </author>
   <link rel="edit-media" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/$value" />
   <content type="binary/octet-stream" src="https://www.myget.org/F/omnisharp/api/v2/package/Microsoft.Extensions.Primitives/1.0.0-rc2-16010" />
   <m:properties>
      <d:Id>Microsoft.Extensions.Primitives</d:Id>
      <d:Version>1.0.0-rc2-16010</d:Version>
      <d:NormalizedVersion>1.0.0-rc2-16010</d:NormalizedVersion>
      <d:Copyright m:null="true" />
      <d:Created m:type="Edm.DateTime">2016-01-22T20:46:59.9523998Z</d:Created>
      <d:Dependencies>::net451|System.Resources.ResourceManager:4.0.0:.NETCore50|System.Runtime:4.0.20:.NETCore50|System.Threading:4.0.10:.NETCore50|System.Runtime:4.0.21-rc2-23706:dotnet5.4|System.Resources.ResourceManager:4.0.1-rc2-23706:dotnet5.4</d:Dependencies>
      <d:Description>ASP.NET 5 primitives.</d:Description>
      <d:DownloadCount m:type="Edm.Int32">15</d:DownloadCount>
      <d:GalleryDetailsUrl>https://www.myget.org/feed/omnisharp/package/nuget/Microsoft.Extensions.Primitives/1.0.0-rc2-16010</d:GalleryDetailsUrl>
      <d:IconUrl m:null="true" />
      <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
      <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
      <d:LastEdited m:type="Edm.DateTime">2016-01-22T20:46:59.9523998Z</d:LastEdited>
      <d:Published m:type="Edm.DateTime">2016-01-22T20:46:59.9523998Z</d:Published>
      <d:LicenseUrl m:null="true" />
      <d:LicenseNames m:null="true" />
      <d:LicenseReportUrl m:null="true" />
      <d:PackageHash>OrfLiJc4So4HHOb7lNJyPSNoFHPM4O8VhqhAg6cdRMlzuFaMF/X4tR43AGDFQH50f30Y2r2eE/4egrWx2gy4xg==</d:PackageHash>
      <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
      <d:PackageSize m:type="Edm.Int64">18238</d:PackageSize>
      <d:ProjectUrl m:null="true" />
      <d:ReleaseNotes m:null="true" />
      <d:ReportAbuseUrl>http://localhost</d:ReportAbuseUrl>
      <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
      <d:Tags m:null="true" />
      <d:Title>Microsoft.Extensions.Primitives</d:Title>
      <d:VersionDownloadCount m:type="Edm.Int32">15</d:VersionDownloadCount>
      <d:IsPrerelease m:type="Edm.Boolean">true</d:IsPrerelease>
      <d:MinClientVersion m:null="true" />
      <d:Language>en-US</d:Language>
   </m:properties>
</entry>"##;

        let package: Package = serde_xml_rs::from_reader(package_serialized.as_bytes()).unwrap();

        assert_eq!(
            package,
            Package {
                id: String::from(
                    "https://www.myget.\
                     org/F/omnisharp/api/v2/Packages(Id='Microsoft.\
                     Extensions.Primitives',Version='1.0.0-rc2-16010')"
                ),
                category: PackageCategory {
                    term: String::from("MyGet.V2FeedPackage")
                },
                title: String::from("Microsoft.Extensions.Primitives"),
                summary: Some(String::from("ASP.NET 5 primitives.")),
                updated_at: String::from("2016-01-22T20:46:59Z"),
                author: Author {
                    name: String::from("Microsoft.Extensions.Primitives")
                },
                content: PackageContent {
                    mime_type: String::from("binary/octet-stream"),
                    url: Url::parse(
                        "https://www.myget.\
                         org/F/omnisharp/api/v2/package/Microsoft.\
                         Extensions.Primitives/1.0.0-rc2-16010"
                    ).unwrap(),
                },
                properties: PackageProperties {
                    id: Some(String::from("Microsoft.Extensions.Primitives")),
                    version: String::from("1.0.0-rc2-16010"),
                    normalized_version: Some(String::from("1.0.0-rc2-16010")),
                    authors: None,
                    copyright: String::from(""),
                    created_at: Some(String::from("2016-01-22T20:46:59.9523998Z")),
                    dependencies: vec![
                        PackageDependency { framework: String::from("net451"), package_id: String::from(""), version: String::from("") },
                        PackageDependency { framework: String::from(".NETCore50"), package_id: String::from("System.Resources.ResourceManager"), version: String::from("4.0.0") },
                        PackageDependency { framework: String::from(".NETCore50"), package_id: String::from("System.Runtime"), version: String::from("4.0.20") },
                        PackageDependency { framework: String::from(".NETCore50"), package_id: String::from("System.Threading"), version: String::from("4.0.10") },
                        PackageDependency { framework: String::from("dotnet5.4"), package_id: String::from("System.Runtime"), version: String::from("4.0.21-rc2-23706") },
                        PackageDependency { framework: String::from("dotnet5.4"), package_id: String::from("System.Resources.ResourceManager"), version: String::from("4.0.1-rc2-23706") },
                    ],
                    description: String::from("ASP.NET 5 primitives."),
                    development_dependency: None,
                    download_count: 15,
                    gallery_details_url: Some(String::from(
                        "https://www.myget.org/feed/omnisharp/package/nuget/Microsoft.Extensions.Primitives/1.0.0-rc2-16010"
                    )),
                    icon_url: Some(String::from("")),
                    is_latest_version: false,
                    is_absolute_latest_version: true,
                    is_prerelease: true,
                    language: Some(String::from("en-US")),
                    last_updated_at: None,
                    owners: None,
                    published_at: Some(String::from("2016-01-22T20:46:59.9523998Z")),
                    package_hash: Some(String::from("OrfLiJc4So4HHOb7lNJyPSNoFHPM4O8VhqhAg6cdRMlzuFaMF/X4tR43AGDFQH50f30Y2r2eE/4egrWx2gy4xg==")),
                    package_hash_algorithm: Some(String::from("SHA512")),
                    package_size: 18238,
                    project_url: String::from(""),
                    report_abuse_url: Some(String::from("http://localhost")),
                    release_notes: String::from(""),
                    require_license_acceptance: false,
                    summary: None,
                    tags: String::from(""),
                    title: String::from("Microsoft.Extensions.Primitives"),
                    version_download_count: 15,
                    minimum_client_version: Some(String::from("")),
                    last_edited_at: Some(String::from("2016-01-22T20:46:59.9523998Z")),
                    license_url: Some(String::from("")),
                    license_names: Some(String::from("")),
                    license_report_url: Some(String::from("")),

                    // ProGet specific fields
                    is_local_package: None,
                    is_proget_hosted: None,
                    is_cached: None,
                    is_listed: None,
                    has_symbols: None,
                    has_source: None,
                },
            }
        );
    }

    #[test]
    fn proget_package() {
        let package_serialized =
r##"<?xml version="1.0" encoding="UTF-8"?>
<entry xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="http://proget/nuget/Default/">
   <id>http://proget/nuget/Default/Packages(Id='Antlr4.Runtime',Version='4.5.3-rc1')</id>
   <title type="text">Antlr4.Runtime</title>
   <summary type="text">The runtime library for parsers generated by the C# target of ANTLR 4.</summary>
   <updated>2016-08-04T12:27:32Z</updated>
   <author>
      <name>Sam Harwell, Terence Parr</name>
   </author>
   <link rel="edit-media" title="Package" href="Packages(Id='Antlr4.Runtime',Version='4.5.3-rc1')/$value" />
   <link rel="edit" title="Package" href="Packages(Id='Antlr4.Runtime',Version='4.5.3-rc1')" />
   <category term="NuGet.Server.DataServices.Package" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
   <content type="application/zip" src="http://proget/nuget/Default/package/Antlr4.Runtime/4.5.3-rc1" />
   <m:properties>
      <d:Version>4.5.3-rc1</d:Version>
      <d:Title>ANTLR 4 Runtime</d:Title>
      <d:RequireLicenseAcceptance m:type="Edm.Boolean">true</d:RequireLicenseAcceptance>
      <d:Description>The runtime library for parsers generated by the C# target of ANTLR 4. This package supports projects targeting .NET 2.0 or newer, and built using Visual Studio 2008 or newer.</d:Description>
      <d:ReleaseNotes>https://github.com/tunnelvisionlabs/antlr4cs/releases/v4.5.3-rc1</d:ReleaseNotes>
      <d:Summary>The runtime library for parsers generated by the C# target of ANTLR 4.</d:Summary>
      <d:ProjectUrl>https://github.com/tunnelvisionlabs/antlr4cs</d:ProjectUrl>
      <d:IconUrl>https://raw.github.com/antlr/website-antlr4/master/images/icons/antlr.png</d:IconUrl>
      <d:LicenseUrl>https://raw.github.com/tunnelvisionlabs/antlr4cs/master/LICENSE.txt</d:LicenseUrl>
      <d:Copyright>Copyright © Sam Harwell 2015</d:Copyright>
      <d:Tags>antlr antlr4 parsing</d:Tags>
      <d:Dependencies />
      <d:IsLocalPackage m:type="Edm.Boolean">true</d:IsLocalPackage>
      <d:Created m:type="Edm.DateTime">2016-08-04T12:27:32.5030000Z</d:Created>
      <d:Published m:type="Edm.DateTime">2016-08-04T12:27:32.5030000Z</d:Published>
      <d:PackageSize m:type="Edm.Int64">1662759</d:PackageSize>
      <d:PackageHash>dPb/HRNYfLKDNFj3K1tlZf+f5gyQq03jE3UjJk9f55YoV0lnXJ8m9hFjhooa+K5VcA/N5/LLiOkPSrM2i+sF3Q==</d:PackageHash>
      <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
      <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">false</d:IsAbsoluteLatestVersion>
      <d:IsProGetHosted m:type="Edm.Boolean">true</d:IsProGetHosted>
      <d:IsPrerelease m:type="Edm.Boolean">true</d:IsPrerelease>
      <d:IsCached m:type="Edm.Boolean">false</d:IsCached>
      <d:NormalizedVersion>4.5.3-rc1</d:NormalizedVersion>
      <d:Listed m:type="Edm.Boolean">true</d:Listed>
      <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
      <d:HasSymbols m:type="Edm.Boolean">false</d:HasSymbols>
      <d:HasSource m:type="Edm.Boolean">false</d:HasSource>
      <d:DownloadCount m:type="Edm.Int32">268</d:DownloadCount>
      <d:VersionDownloadCount m:type="Edm.Int32">116</d:VersionDownloadCount>
   </m:properties>
</entry>"##;

        let package: Package = serde_xml_rs::from_reader(package_serialized.as_bytes()).unwrap();

        assert_eq!(
            package,
            Package {
                id: String::from(
                    "http://proget/nuget/Default/Packages(Id='Antlr4.\
                     Runtime',Version='4.5.3-rc1')"
                ),
                category: PackageCategory {
                    term: String::from("NuGet.Server.DataServices.Package"),
                },
                title: String::from("Antlr4.Runtime"),
                summary: Some(String::from(
                    "The runtime library for parsers generated by \
                     the C# target of ANTLR 4."
                )),
                updated_at: String::from("2016-08-04T12:27:32Z"),
                author: Author {
                    name: String::from("Sam Harwell, Terence Parr")
                },
                content: PackageContent {
                    mime_type: String::from("application/zip"),
                    url: Url::parse(
                        "http://proget/nuget/Default/package/Antlr4.\
                         Runtime/4.5.3-rc1"
                    ).unwrap(),
                },
                properties: PackageProperties {
                    id: None,
                    version: String::from("4.5.3-rc1"),
                    normalized_version: Some(String::from("4.5.3-rc1")),
                    authors: None,
                    copyright: String::from("Copyright © Sam Harwell 2015"),
                    created_at: Some(String::from("2016-08-04T12:27:32.5030000Z")),
                    dependencies: vec![],
                    description: String::from(
                        "The runtime library for parsers generated \
                         by the C# target of ANTLR 4. This package \
                         supports projects targeting .NET 2.0 or \
                         newer, and built using Visual Studio 2008 \
                         or newer."
                    ),
                    development_dependency: None,
                    owners: None,
                    download_count: 268,
                    gallery_details_url: None,
                    icon_url: Some(String::from("https://raw.github.com/antlr/website-antlr4/master/images/icons/antlr.png")),
                    is_latest_version: false,
                    is_absolute_latest_version: false,
                    is_prerelease: true,
                    language: None,
                    last_updated_at: None,
                    published_at: Some(String::from("2016-08-04T12:27:32.5030000Z")),
                    package_hash: Some(String::from("dPb/HRNYfLKDNFj3K1tlZf+f5gyQq03jE3UjJk9f55YoV0lnXJ8m9hFjhooa+K5VcA/N5/LLiOkPSrM2i+sF3Q==")),
                    package_hash_algorithm: Some(String::from("SHA512")),
                    package_size: 1662759,
                    project_url: String::from(
                        "https://github.\
                         com/tunnelvisionlabs/antlr4cs"
                    ),
                    report_abuse_url: None,
                    release_notes: String::from(
                        "https://github.\
                         com/tunnelvisionlabs/antlr4cs/releases/v4.\
                         5.3-rc1"
                    ),
                    require_license_acceptance: true,
                    summary: Some(String::from(
                        "The runtime library for parsers \
                         generated by the C# target of ANTLR 4."
                    )),
                    tags: String::from("antlr antlr4 parsing"),
                    title: String::from("ANTLR 4 Runtime"),
                    version_download_count: 116,
                    minimum_client_version: None,
                    last_edited_at: None,
                    license_url: Some(String::from("https://raw.github.com/tunnelvisionlabs/antlr4cs/master/LICENSE.txt")),
                    license_names: None,
                    license_report_url: None,

                    // ProGet specific fields
                    is_local_package: Some(true),
                    is_proget_hosted: Some(true),
                    is_cached: Some(false),
                    is_listed: Some(true),
                    has_symbols: Some(false),
                    has_source: Some(false),
                },
            }
        );
    }

    #[test]
    fn bintray_package() {
        // From https://api.bintray.com/nuget/fint/nuget/Packages(Id='fint-eventsource',Version='0.4.0.1')
        let package_serialized = r##"<entry xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="https://api.bintray.com/nuget/fint/nuget/">
   <id>https://api.bintray.com/nuget/fint/nuget/Packages(Id='fint-eventsource',Version='0.4.0.1')</id>
   <title type="text">fint-eventsource</title>
   <summary type="text">An eventsource(Server-Sent Events client) implementation for .Net.</summary>
   <updated>2017-05-04T11:03:27Z</updated>
   <author>
      <name>erizet</name>
   </author>
   <link rel="edit" title="V2FeedPackage" href="Packages(Id='fint-eventsource',Version='0.4.0.1')" />
   <link rel="self" title="V2FeedPackage" href="Packages(Id='fint-eventsource',Version='0.4.0.1')" />
   <category term="NuGetGallery.OData.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
   <content type="application/zip" src="https://api.bintray.com/nuget/fint/nuget/Download/fint-eventsource/0.4.0.1" />
   <m:properties>
      <d:lastUpdated>2017-05-04T11:03:27</d:lastUpdated>
      <d:Version>0.4.0.1</d:Version>
      <d:Copyright>Copyright 2017</d:Copyright>
      <d:Created m:type="Edm.DateTime">2017-05-04T11:03:28</d:Created>
      <d:Dependencies>slf4net:0.1.32.1:</d:Dependencies>
      <d:Description>An eventsource(Server-Sent Events client) implementation for .Net.</d:Description>
      <d:DownloadCount m:type="Edm.Int32">0</d:DownloadCount>
      <d:IsLatestVersion m:type="Edm.Boolean">true</d:IsLatestVersion>
      <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
      <d:IsPrerelease m:type="Edm.Boolean">false</d:IsPrerelease>
      <d:Language m:null="true" />
      <d:Published m:type="Edm.DateTime">2017-05-04T11:03:28</d:Published>
      <d:PackageHash>otpBPpuwCOPT5J12azb9MvStj2+WA1nqX/8aAkNjO7Wuohsg/M+d17l1M6k9D4c+B4k6/3XC376eMmbb7TG68A==</d:PackageHash>
      <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
      <d:PackageSize m:type="Edm.Int64">9989</d:PackageSize>
      <d:ProjectUrl>https://github.com/fintprosjektet</d:ProjectUrl>
      <d:ReleaseNotes />
      <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
      <d:Tags>fint sse server-sent</d:Tags>
      <d:Title>EventSource4Net</d:Title>
      <d:VersionDownloadCount m:type="Edm.Int32">0</d:VersionDownloadCount>
      <d:Authors>erizet</d:Authors>
      <d:MinClientVersion m:null="true" />
   </m:properties>
</entry>"##;

        let package: Package = serde_xml_rs::from_reader(package_serialized.as_bytes()).unwrap();

        assert_eq!(
            package,
            Package {
                id: String::from(
                    "https://api.bintray.\
                     com/nuget/fint/nuget/Packages(Id='fint-eventsource',\
                     Version='0.4.0.1')"
                ),
                category: PackageCategory {
                    term: String::from("NuGetGallery.OData.V2FeedPackage"),
                },
                title: String::from("fint-eventsource"),
                summary: Some(String::from(
                    "An eventsource(Server-Sent Events client) \
                     implementation for .Net."
                )),
                updated_at: String::from("2017-05-04T11:03:27Z"),
                author: Author {
                    name: String::from("erizet")
                },
                content: PackageContent {
                    mime_type: String::from("application/zip"),
                    url: Url::parse(
                        "https://api.bintray.\
                         com/nuget/fint/nuget/Download/fint-eventsource/0.4.\
                         0.1"
                    ).unwrap(),
                },
                properties: PackageProperties {
                    id: None,
                    version: String::from("0.4.0.1"),
                    normalized_version: None,
                    authors: Some(String::from("erizet")),
                    copyright: String::from("Copyright 2017"),
                    created_at: Some(String::from("2017-05-04T11:03:28")),
                    dependencies: vec![
                        PackageDependency { framework: String::from(""), package_id: String::from("slf4net"), version: String::from("0.1.32.1") },
                    ],
                    development_dependency: None,
                    owners: None,
                    description: String::from(
                        "An eventsource(Server-Sent Events client) \
                         implementation for .Net."
                    ),
                    download_count: 0,
                    gallery_details_url: None,
                    icon_url: None,
                    is_latest_version: true,
                    is_absolute_latest_version: true,
                    is_prerelease: false,
                    language: Some(String::from("")),
                    last_updated_at: None,
                    published_at: Some(String::from("2017-05-04T11:03:28")),
                    package_hash: Some(String::from("otpBPpuwCOPT5J12azb9MvStj2+WA1nqX/8aAkNjO7Wuohsg/M+d17l1M6k9D4c+B4k6/3XC376eMmbb7TG68A==")),
                    package_hash_algorithm: Some(String::from("SHA512")),
                    package_size: 9989,
                    project_url: String::from("https://github.com/fintprosjektet"),
                    report_abuse_url: None,
                    release_notes: String::from(""),
                    require_license_acceptance: false,
                    summary: None,
                    tags: String::from("fint sse server-sent"),
                    title: String::from("EventSource4Net"),
                    version_download_count: 0,
                    minimum_client_version: Some(String::from("")),
                    last_edited_at: None,
                    license_url: None,
                    license_names: None,
                    license_report_url: None,

                    // ProGet specific fields
                    is_local_package: None,
                    is_proget_hosted: None,
                    is_cached: None,
                    is_listed: None,
                    has_symbols: None,
                    has_source: None,
                },
            }
        );
    }
}
