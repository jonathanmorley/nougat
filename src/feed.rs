use package::Package;
use std::fmt;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Feed {
    pub id: String,
    pub title: String,
    #[serde(rename = "updated")]
    pub updated_at: String,
    #[serde(rename = "link", default)]
    pub links: Vec<Link>,
    #[serde(rename = "entry", default)]
    pub packages: Vec<Package>,
}

#[serde(tag = "rel", content = "href", rename_all = "kebab-case")]
#[derive(Debug, Deserialize, PartialEq)]
pub enum Link {
    Edit(String),
    EditMedia(String),
    #[serde(rename = "self")]
    _Self(String),
    #[serde(rename = "http://schemas.microsoft.com/ado/2007/08/dataservices/related/Screenshots")]
    Screenshots(String),
}

impl Feed {
    pub fn subtract(&self, other: &Feed) -> Vec<&Package> {
        let mut diff = Vec::new();

        'outer: for package in &self.packages {
            for other_package in &other.packages {
                if package == other_package {
                    continue 'outer;
                }
            }

            diff.push(package);
        }

        diff
    }
}

impl fmt::Display for Feed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(for package in &self.packages {
            match writeln!(f, "{}", package) {
                Ok(_) => continue,
                err => return err,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_xml_rs;

    use super::*;

    #[test]
    fn no_package_feed() {
        // From https://www.nuget.org/api/v2/Packages()
        let feed_serialized = r##"<?xml version="1.0" encoding="utf-8"?>
    <feed xml:base="https://www.nuget.org/api/v2" xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xmlns:georss="http://www.georss.org/georss" xmlns:gml="http://www.opengis.net/gml">
    <id>http://schemas.datacontract.org/2004/07/</id>
    <title />
    <updated>2017-06-14T19:49:57Z</updated>
    <link rel="self" href="https://www.nuget.org/api/v2/Packages" />
</feed>"##;

        let feed: Feed = serde_xml_rs::from_reader(feed_serialized.as_bytes()).unwrap();

        assert_eq!(
            feed,
            Feed {
                id: String::from("http://schemas.datacontract.org/2004/07/"),
                title: String::from(""),
                updated_at: String::from("2017-06-14T19:49:57Z"),
                links: vec![Link::_Self(String::from(
                    "https://www.nuget.\
                     org/api/v2/Packages"
                ))],
                packages: vec![],
            }
        );
    }

    #[test]
    fn nuget_gallery_feed() {
        // From https://www.nuget.org/api/v2/Packages()
        let feed_serialized =
r##"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:georss="http://www.georss.org/georss" xmlns:gml="http://www.opengis.net/gml" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="https://www.nuget.org/api/v2">
   <id>http://schemas.datacontract.org/2004/07/</id>
   <title />
   <updated>2017-06-16T15:13:03Z</updated>
   <link rel="self" href="https://www.nuget.org/api/v2/Packages" />
   <entry>
      <id>https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')</id>
      <category term="NuGetGallery.OData.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
      <link rel="edit" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
      <link rel="self" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
      <title type="text">_51Wp.AccountSdk</title>
      <updated>2015-12-15T07:05:02Z</updated>
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
         <d:Created m:type="Edm.DateTime">2015-12-15T07:05:02.15</d:Created>
         <d:Dependencies />
         <d:Description>My package description.</d:Description>
         <d:DownloadCount m:type="Edm.Int32">2195</d:DownloadCount>
         <d:GalleryDetailsUrl>https://www.nuget.org/packages/_51Wp.AccountSdk/1.0.0</d:GalleryDetailsUrl>
         <d:IconUrl m:null="true" />
         <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
         <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">false</d:IsAbsoluteLatestVersion>
         <d:IsPrerelease m:type="Edm.Boolean">false</d:IsPrerelease>
         <d:Language m:null="true" />
         <d:LastUpdated m:type="Edm.DateTime">2015-12-15T07:05:02.15</d:LastUpdated>
         <d:Published m:type="Edm.DateTime">1900-01-01T00:00:00</d:Published>
         <d:PackageHash>CwkBmkdSDYieaAgZxyrFizngyNfBB76piK7KFe7T8WgRH7opJZLiz6LdO3CCHp0u0E2GVazgbzAPJG+PNpzT1g==</d:PackageHash>
         <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
         <d:PackageSize m:type="Edm.Int64">212213</d:PackageSize>
         <d:ProjectUrl m:null="true" />
         <d:ReportAbuseUrl>https://www.nuget.org/packages/_51Wp.AccountSdk/1.0.0/ReportAbuse</d:ReportAbuseUrl>
         <d:ReleaseNotes m:null="true" />
         <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
         <d:Summary m:null="true" />
         <d:Tags m:null="true" />
         <d:Title m:null="true" />
         <d:VersionDownloadCount m:type="Edm.Int32">2195</d:VersionDownloadCount>
         <d:MinClientVersion m:null="true" />
         <d:LastEdited m:type="Edm.DateTime">2015-12-15T14:58:39.043</d:LastEdited>
         <d:LicenseUrl m:null="true" />
         <d:LicenseNames m:null="true" />
         <d:LicenseReportUrl m:null="true" />
      </m:properties>
   </entry>
</feed>"##;

        let feed: Feed = serde_xml_rs::from_reader(feed_serialized.as_bytes()).unwrap();

        assert_eq!(
            feed.id,
            String::from("http://schemas.datacontract.org/2004/07/")
        );
        assert_eq!(feed.title, String::from(""));
        assert_eq!(feed.updated_at, String::from("2017-06-16T15:13:03Z"));
        assert_eq!(
            feed.links,
            vec![Link::_Self(String::from(
                "https://www.nuget.org/api/v2/Packages"
            ))]
        );

        assert_eq!(format!("{}", feed), "_51Wp.AccountSdk 1.0.0\n");
    }

    #[test]
    fn myget_feed() {
        // From https://www.myget.org/F/omnisharp/api/v2/Packages()
        let feed_serialized =
r##"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="https://www.myget.org/F/omnisharp/api/v2/">
   <id>https://www.myget.org/F/omnisharp/api/v2/Packages</id>
   <title type="text">Packages</title>
   <updated>2017-06-16T15:14:44Z</updated>
   <link rel="self" title="Packages" href="Packages" />
   <entry>
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
         <d:Created m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Created>
         <d:Dependencies>::net451|System.Resources.ResourceManager:4.0.0:.NETCore50|System.Runtime:4.0.20:.NETCore50|System.Threading:4.0.10:.NETCore50|System.Runtime:4.0.21-rc2-23706:dotnet5.4|System.Resources.ResourceManager:4.0.1-rc2-23706:dotnet5.4</d:Dependencies>
         <d:Description>ASP.NET 5 primitives.</d:Description>
         <d:DownloadCount m:type="Edm.Int32">15</d:DownloadCount>
         <d:GalleryDetailsUrl>https://www.myget.org/feed/omnisharp/package/nuget/Microsoft.Extensions.Primitives/1.0.0-rc2-16010</d:GalleryDetailsUrl>
         <d:IconUrl m:null="true" />
         <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
         <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
         <d:LastEdited m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:LastEdited>
         <d:Published m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Published>
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
   </entry>
</feed>"##;

        let feed: Feed = serde_xml_rs::from_reader(feed_serialized.as_bytes()).unwrap();

        assert_eq!(
            feed.id,
            String::from("https://www.myget.org/F/omnisharp/api/v2/Packages")
        );
        assert_eq!(feed.title, String::from("Packages"));
        assert_eq!(feed.updated_at, String::from("2017-06-16T15:14:44Z"));
        assert_eq!(feed.links, vec![Link::_Self(String::from("Packages"))]);

        assert_eq!(
            format!("{}", feed),
            "Microsoft.Extensions.Primitives 1.0.0-rc2-16010\n"
        );
    }

    #[test]
    fn proget_feed() {
        let feed_serialized =
r##"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="http://proget/nuget/Default/">
   <title type="text">Packages</title>
   <id>http://proget/nuget/Default/Packages()/</id>
   <updated>2017-06-16T15:27:31Z</updated>
   <link rel="self" title="Packages" href="Packages" />
   <entry>
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
   </entry>
</feed>"##;

        let feed: Feed = serde_xml_rs::from_reader(feed_serialized.as_bytes()).unwrap();

        assert_eq!(
            feed.id,
            String::from("http://proget/nuget/Default/Packages()/")
        );
        assert_eq!(feed.title, String::from("Packages"));
        assert_eq!(feed.updated_at, String::from("2017-06-16T15:27:31Z"));
        assert_eq!(feed.links, vec![Link::_Self(String::from("Packages"))]);

        assert_eq!(format!("{}", feed), "Antlr4.Runtime 4.5.3-rc1\n");
    }

    #[test]
    fn bintray_feed() {
        // From https://api.bintray.com/nuget/fint/nuget/Packages()
        let feed_serialized =
r##"<feed xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xml:base="https://api.bintray.com/nuget/fint/nuget/">
   <title type="text" />
   <id>http://schemas.datacontract.org/2004/07/</id>
   <updated>2017-06-16T16:06:10Z</updated>
   <link rel="self" title="Packages" href="Packages" />
   <entry>
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
   </entry>
</feed>"##;

        let feed: Feed = serde_xml_rs::from_reader(feed_serialized.as_bytes()).unwrap();

        assert_eq!(
            feed.id,
            String::from("http://schemas.datacontract.org/2004/07/")
        );
        assert_eq!(feed.title, String::from(""));
        assert_eq!(feed.updated_at, String::from("2017-06-16T16:06:10Z"));
        assert_eq!(feed.links, vec![Link::_Self(String::from("Packages"))]);

        assert_eq!(format!("{}", feed), "fint-eventsource 0.4.0.1\n");
    }

    #[test]
    fn many_package_feed() {
        let feed_serialized =
r##"<?xml version="1.0" encoding="utf-8"?>
    <feed xml:base="https://www.nuget.org/api/v2" xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xmlns:georss="http://www.georss.org/georss" xmlns:gml="http://www.opengis.net/gml">
    <id>http://schemas.datacontract.org/2004/07/</id>
    <title />
    <updated>2017-06-14T19:49:57Z</updated>
    <link rel="self" href="https://www.nuget.org/api/v2/Packages" />
    <entry>
        <id>https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')</id>
        <category term="NuGetGallery.OData.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <link rel="edit" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
        <link rel="self" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
        <title type="text">_51Wp.AccountSdk</title>
        <updated>2015-12-15T15:05:02Z</updated>
        <author><name>authors</name></author>
        <content type="application/zip" src="https://www.nuget.org/api/v2/package/_51Wp.AccountSdk/1.0.0" />
        <m:properties>
            <d:Id>_51Wp.AccountSdk</d:Id>
            <d:Version>1.0.0</d:Version>
            <d:NormalizedVersion>1.0.0</d:NormalizedVersion>
            <d:Authors>authors</d:Authors>
            <d:Copyright m:null="true" />
            <d:Created m:type="Edm.DateTime">2015-12-15T15:05:02.15Z</d:Created>
            <d:Dependencies></d:Dependencies>
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
            <d:Summary></d:Summary>
            <d:Tags></d:Tags>
            <d:Title>_51Wp.AccountSdk</d:Title>
            <d:VersionDownloadCount m:type="Edm.Int32">2195</d:VersionDownloadCount>
            <d:MinClientVersion m:null="true" />
            <d:LastEdited m:type="Edm.DateTime">2015-12-15T22:58:39.043Z</d:LastEdited>
            <d:LicenseUrl m:null="true" />
            <d:LicenseNames m:null="true" />
            <d:LicenseReportUrl m:null="true" />
        </m:properties>
    </entry>
    <entry>
        <id>https://www.myget.org/F/omnisharp/api/v2/Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')</id>
        <category term="MyGet.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <link rel="edit" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')" />
        <link rel="http://schemas.microsoft.com/ado/2007/08/dataservices/related/Screenshots" type="application/atom+xml;type=feed" title="Screenshots" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/Screenshots" />
        <title type="text">Microsoft.Extensions.Primitives</title>
        <summary type="text">ASP.NET 5 primitives.</summary>
        <updated>2016-01-22T20:46:59Z</updated>
        <author><name>Microsoft.Extensions.Primitives</name></author>
        <link rel="edit-media" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/$value" />
        <content type="binary/octet-stream" src="https://www.myget.org/F/omnisharp/api/v2/package/Microsoft.Extensions.Primitives/1.0.0-rc2-16010" />
        <m:properties>
          <d:Id>Microsoft.Extensions.Primitives</d:Id>
          <d:Version>1.0.0-rc2-16010</d:Version>
          <d:NormalizedVersion>1.0.0-rc2-16010</d:NormalizedVersion>
          <d:Copyright m:null="true" />
          <d:Created m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Created>
          <d:Dependencies>::net451|System.Resources.ResourceManager:4.0.0:.NETCore50|System.Runtime:4.0.20:.NETCore50|System.Threading:4.0.10:.NETCore50|System.Runtime:4.0.21-rc2-23706:dotnet5.4|System.Resources.ResourceManager:4.0.1-rc2-23706:dotnet5.4</d:Dependencies>
          <d:Description>ASP.NET 5 primitives.</d:Description>
          <d:DownloadCount m:type="Edm.Int32">15</d:DownloadCount>
          <d:GalleryDetailsUrl>https://www.myget.org/feed/omnisharp/package/nuget/Microsoft.Extensions.Primitives/1.0.0-rc2-16010</d:GalleryDetailsUrl>
          <d:IconUrl m:null="true" />
          <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
          <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
          <d:LastEdited m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:LastEdited>
          <d:Published m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Published>
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
    </entry>
    <entry>
        <id>http://proget/nuget/Default/Packages(Id='Algolia.Search',Version='3.6.4')</id>
        <title type="text">Algolia.Search</title>
        <summary type="text"></summary>
        <updated>2017-06-06T20:15:27Z</updated>
        <author><name>Algolia</name></author>
        <link rel="edit-media" title="Package" href="Packages(Id='Algolia.Search',Version='3.6.4')/$value" />
        <link rel="edit" title="Package" href="Packages(Id='Algolia.Search',Version='3.6.4')" />
        <category term="NuGet.Server.DataServices.Package" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <content type="application/zip" src="http://proget/nuget/Default/package/Algolia.Search/3.6.4" />
        <m:properties xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata">
          <d:Version>3.6.4</d:Version>
          <d:Title>Algolia Search API Client for C#</d:Title>
          <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
          <d:Description>Algolia Search API Client for C#</d:Description>
          <d:ReleaseNotes>Changed to single project to support .NET 4.0, .NET 4.5, ASP.NET vNext 1.0, Mono 4.5, Windows 8, Windows 8.1, Windows Phone 8.1, Xamarin iOS, and Xamarin Android.</d:ReleaseNotes>
          <d:Summary></d:Summary>
          <d:ProjectUrl>https://github.com/algolia/algoliasearch-client-csharp</d:ProjectUrl>
          <d:IconUrl>http://www.algolia.com/download/logo.png</d:IconUrl>
          <d:LicenseUrl>https://github.com/algolia/algoliasearch-client-csharp/blob/master/LICENSE.TXT</d:LicenseUrl>
          <d:Copyright>Copyright 2015</d:Copyright>
          <d:Tags>Search Engine Service Instant Typo-Tolerance Realtime</d:Tags>
          <d:Dependencies>Microsoft.Bcl.Async:1.0.168|Microsoft.Net.Http:2.2.29|Newtonsoft.Json:6.0.8|PCLCrypto:1.0.1.15115</d:Dependencies>
          <d:IsLocalPackage m:type="Edm.Boolean">true</d:IsLocalPackage>
          <d:Created m:type="Edm.DateTime">2017-06-06T20:15:27.0800000Z</d:Created>
          <d:Published m:type="Edm.DateTime">2017-06-06T20:15:27.0800000Z</d:Published>
          <d:PackageSize m:type="Edm.Int64">38807</d:PackageSize>
          <d:PackageHash>5eypGYXO4aHaXsSSgkFu0GkB5ijqqYbSW+l5yBnFxHinvhZl4DNBk1AUMSrFvnqD1aTAtRF4qYbW8MoXqcr+kA==</d:PackageHash>
          <d:IsLatestVersion m:type="Edm.Boolean">true</d:IsLatestVersion>
          <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
          <d:IsProGetHosted m:type="Edm.Boolean">true</d:IsProGetHosted>
          <d:IsPrerelease m:type="Edm.Boolean">false</d:IsPrerelease>
          <d:IsCached m:type="Edm.Boolean">false</d:IsCached>
          <d:NormalizedVersion>3.6.4</d:NormalizedVersion>
          <d:Listed m:type="Edm.Boolean">true</d:Listed>
          <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
          <d:HasSymbols m:type="Edm.Boolean">false</d:HasSymbols>
          <d:HasSource m:type="Edm.Boolean">false</d:HasSource>
          <d:DownloadCount m:type="Edm.Int32">0</d:DownloadCount>
          <d:VersionDownloadCount m:type="Edm.Int32">0</d:VersionDownloadCount>
        </m:properties>
    </entry>
    <entry>
        <id>https://api.bintray.com/nuget/fint/nuget/Packages(Id='fint-eventsource',Version='0.4.0.1')</id>
        <title type="text">fint-eventsource</title>
        <summary type="text">An eventsource(Server-Sent Events client) implementation for .Net.</summary>
        <updated>2017-05-04T11:03:27Z</updated>
        <author><name>erizet</name></author>
        <link rel="edit" title="V2FeedPackage" href="Packages(Id='fint-eventsource',Version='0.4.0.1')"/>
        <link rel="self" title="V2FeedPackage" href="Packages(Id='fint-eventsource',Version='0.4.0.1')"/>
        <category term="NuGetGallery.OData.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme"/>
        <content type="application/zip" src="https://api.bintray.com/nuget/fint/nuget/Download/fint-eventsource/0.4.0.1"/>
        <m:properties xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices">
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
          <d:Language m:null="true"/>
          <d:Published m:type="Edm.DateTime">2017-05-04T11:03:28</d:Published>
          <d:PackageHash>otpBPpuwCOPT5J12azb9MvStj2+WA1nqX/8aAkNjO7Wuohsg/M+d17l1M6k9D4c+B4k6/3XC376eMmbb7TG68A==</d:PackageHash>
          <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
          <d:PackageSize m:type="Edm.Int64">9989</d:PackageSize>
          <d:ProjectUrl>https://github.com/fintprosjektet</d:ProjectUrl>
          <d:ReleaseNotes></d:ReleaseNotes>
          <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
          <d:Tags>fint sse server-sent</d:Tags>
          <d:Title>EventSource4Net</d:Title>
          <d:VersionDownloadCount m:type="Edm.Int32">0</d:VersionDownloadCount>
          <d:Authors>erizet</d:Authors>
          <d:MinClientVersion m:null="true"/>
        </m:properties>
    </entry>
</feed>"##;

        let feed: Feed = serde_xml_rs::from_reader(feed_serialized.as_bytes()).unwrap();

        assert_eq!(
            format!("{}", feed),
            "_51Wp.AccountSdk 1.0.0\nMicrosoft.Extensions.Primitives \
             1.0.0-rc2-16010\nAlgolia.Search 3.6.4\nfint-eventsource 0.4.0.1\n"
        );
    }

    #[test]
    fn subtraction() {
        let feed_1_serialized =
r##"<?xml version="1.0" encoding="utf-8"?>
    <feed xml:base="https://www.nuget.org/api/v2" xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xmlns:georss="http://www.georss.org/georss" xmlns:gml="http://www.opengis.net/gml">
    <id>http://schemas.datacontract.org/2004/07/</id>
    <title />
    <updated>2017-06-14T19:49:57Z</updated>
    <link rel="self" href="https://www.nuget.org/api/v2/Packages" />
    <entry>
        <id>https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')</id>
        <category term="NuGetGallery.OData.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <link rel="edit" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
        <link rel="self" href="https://www.nuget.org/api/v2/Packages(Id='_51Wp.AccountSdk',Version='1.0.0')" />
        <title type="text">_51Wp.AccountSdk</title>
        <updated>2015-12-15T15:05:02Z</updated>
        <author><name>authors</name></author>
        <content type="application/zip" src="https://www.nuget.org/api/v2/package/_51Wp.AccountSdk/1.0.0" />
        <m:properties>
            <d:Id>_51Wp.AccountSdk</d:Id>
            <d:Version>1.0.0</d:Version>
            <d:NormalizedVersion>1.0.0</d:NormalizedVersion>
            <d:Authors>authors</d:Authors>
            <d:Copyright m:null="true" />
            <d:Created m:type="Edm.DateTime">2015-12-15T15:05:02.15Z</d:Created>
            <d:Dependencies></d:Dependencies>
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
            <d:Summary></d:Summary>
            <d:Tags></d:Tags>
            <d:Title>_51Wp.AccountSdk</d:Title>
            <d:VersionDownloadCount m:type="Edm.Int32">2195</d:VersionDownloadCount>
            <d:MinClientVersion m:null="true" />
            <d:LastEdited m:type="Edm.DateTime">2015-12-15T22:58:39.043Z</d:LastEdited>
            <d:LicenseUrl m:null="true" />
            <d:LicenseNames m:null="true" />
            <d:LicenseReportUrl m:null="true" />
        </m:properties>
    </entry>
    <entry>
        <id>https://www.myget.org/F/omnisharp/api/v2/Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')</id>
        <category term="MyGet.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <link rel="edit" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')" />
        <link rel="http://schemas.microsoft.com/ado/2007/08/dataservices/related/Screenshots" type="application/atom+xml;type=feed" title="Screenshots" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/Screenshots" />
        <title type="text">Microsoft.Extensions.Primitives</title>
        <summary type="text">ASP.NET 5 primitives.</summary>
        <updated>2016-01-22T20:46:59Z</updated>
        <author><name>Microsoft.Extensions.Primitives</name></author>
        <link rel="edit-media" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/$value" />
        <content type="binary/octet-stream" src="https://www.myget.org/F/omnisharp/api/v2/package/Microsoft.Extensions.Primitives/1.0.0-rc2-16010" />
        <m:properties>
          <d:Id>Microsoft.Extensions.Primitives</d:Id>
          <d:Version>1.0.0-rc2-16010</d:Version>
          <d:NormalizedVersion>1.0.0-rc2-16010</d:NormalizedVersion>
          <d:Copyright m:null="true" />
          <d:Created m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Created>
          <d:Dependencies>::net451|System.Resources.ResourceManager:4.0.0:.NETCore50|System.Runtime:4.0.20:.NETCore50|System.Threading:4.0.10:.NETCore50|System.Runtime:4.0.21-rc2-23706:dotnet5.4|System.Resources.ResourceManager:4.0.1-rc2-23706:dotnet5.4</d:Dependencies>
          <d:Description>ASP.NET 5 primitives.</d:Description>
          <d:DownloadCount m:type="Edm.Int32">15</d:DownloadCount>
          <d:GalleryDetailsUrl>https://www.myget.org/feed/omnisharp/package/nuget/Microsoft.Extensions.Primitives/1.0.0-rc2-16010</d:GalleryDetailsUrl>
          <d:IconUrl m:null="true" />
          <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
          <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
          <d:LastEdited m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:LastEdited>
          <d:Published m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Published>
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
    </entry>
    <entry>
        <id>http://proget/nuget/Default/Packages(Id='Algolia.Search',Version='3.6.4')</id>
        <title type="text">Algolia.Search</title>
        <summary type="text"></summary>
        <updated>2017-06-06T20:15:27Z</updated>
        <author><name>Algolia</name></author>
        <link rel="edit-media" title="Package" href="Packages(Id='Algolia.Search',Version='3.6.4')/$value" />
        <link rel="edit" title="Package" href="Packages(Id='Algolia.Search',Version='3.6.4')" />
        <category term="NuGet.Server.DataServices.Package" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <content type="application/zip" src="http://proget/nuget/Default/package/Algolia.Search/3.6.4" />
        <m:properties xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata">
          <d:Version>3.6.4</d:Version>
          <d:Title>Algolia Search API Client for C#</d:Title>
          <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
          <d:Description>Algolia Search API Client for C#</d:Description>
          <d:ReleaseNotes>Changed to single project to support .NET 4.0, .NET 4.5, ASP.NET vNext 1.0, Mono 4.5, Windows 8, Windows 8.1, Windows Phone 8.1, Xamarin iOS, and Xamarin Android.</d:ReleaseNotes>
          <d:Summary></d:Summary>
          <d:ProjectUrl>https://github.com/algolia/algoliasearch-client-csharp</d:ProjectUrl>
          <d:IconUrl>http://www.algolia.com/download/logo.png</d:IconUrl>
          <d:LicenseUrl>https://github.com/algolia/algoliasearch-client-csharp/blob/master/LICENSE.TXT</d:LicenseUrl>
          <d:Copyright>Copyright 2015</d:Copyright>
          <d:Tags>Search Engine Service Instant Typo-Tolerance Realtime</d:Tags>
          <d:Dependencies>Microsoft.Bcl.Async:1.0.168|Microsoft.Net.Http:2.2.29|Newtonsoft.Json:6.0.8|PCLCrypto:1.0.1.15115</d:Dependencies>
          <d:IsLocalPackage m:type="Edm.Boolean">true</d:IsLocalPackage>
          <d:Created m:type="Edm.DateTime">2017-06-06T20:15:27.0800000Z</d:Created>
          <d:Published m:type="Edm.DateTime">2017-06-06T20:15:27.0800000Z</d:Published>
          <d:PackageSize m:type="Edm.Int64">38807</d:PackageSize>
          <d:PackageHash>5eypGYXO4aHaXsSSgkFu0GkB5ijqqYbSW+l5yBnFxHinvhZl4DNBk1AUMSrFvnqD1aTAtRF4qYbW8MoXqcr+kA==</d:PackageHash>
          <d:IsLatestVersion m:type="Edm.Boolean">true</d:IsLatestVersion>
          <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
          <d:IsProGetHosted m:type="Edm.Boolean">true</d:IsProGetHosted>
          <d:IsPrerelease m:type="Edm.Boolean">false</d:IsPrerelease>
          <d:IsCached m:type="Edm.Boolean">false</d:IsCached>
          <d:NormalizedVersion>3.6.4</d:NormalizedVersion>
          <d:Listed m:type="Edm.Boolean">true</d:Listed>
          <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
          <d:HasSymbols m:type="Edm.Boolean">false</d:HasSymbols>
          <d:HasSource m:type="Edm.Boolean">false</d:HasSource>
          <d:DownloadCount m:type="Edm.Int32">0</d:DownloadCount>
          <d:VersionDownloadCount m:type="Edm.Int32">0</d:VersionDownloadCount>
        </m:properties>
    </entry>
</feed>"##;

        let feed_2_serialized =
r##"<?xml version="1.0" encoding="utf-8"?>
<feed xml:base="https://www.nuget.org/api/v2" xmlns="http://www.w3.org/2005/Atom" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xmlns:georss="http://www.georss.org/georss" xmlns:gml="http://www.opengis.net/gml">
    <id>http://schemas.datacontract.org/2004/07/</id>
    <title />
    <updated>2017-06-14T19:49:57Z</updated>
    <link rel="self" href="https://www.nuget.org/api/v2/Packages" />
    <entry>
        <id>https://www.myget.org/F/omnisharp/api/v2/Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')</id>
        <category term="MyGet.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <link rel="edit" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')" />
        <link rel="http://schemas.microsoft.com/ado/2007/08/dataservices/related/Screenshots" type="application/atom+xml;type=feed" title="Screenshots" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/Screenshots" />
        <title type="text">Microsoft.Extensions.Primitives</title>
        <summary type="text">ASP.NET 5 primitives.</summary>
        <updated>2016-01-22T20:46:59Z</updated>
        <author><name>Microsoft.Extensions.Primitives</name></author>
        <link rel="edit-media" title="V2FeedPackage" href="Packages(Id='Microsoft.Extensions.Primitives',Version='1.0.0-rc2-16010')/$value" />
        <content type="binary/octet-stream" src="https://www.myget.org/F/omnisharp/api/v2/package/Microsoft.Extensions.Primitives/1.0.0-rc2-16010" />
        <m:properties>
          <d:Id>Microsoft.Extensions.Primitives</d:Id>
          <d:Version>1.0.0-rc2-16010</d:Version>
          <d:NormalizedVersion>1.0.0-rc2-16010</d:NormalizedVersion>
          <d:Copyright m:null="true" />
          <d:Created m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Created>
          <d:Dependencies>::net451|System.Resources.ResourceManager:4.0.0:.NETCore50|System.Runtime:4.0.20:.NETCore50|System.Threading:4.0.10:.NETCore50|System.Runtime:4.0.21-rc2-23706:dotnet5.4|System.Resources.ResourceManager:4.0.1-rc2-23706:dotnet5.4</d:Dependencies>
          <d:Description>ASP.NET 5 primitives.</d:Description>
          <d:DownloadCount m:type="Edm.Int32">15</d:DownloadCount>
          <d:GalleryDetailsUrl>https://www.myget.org/feed/omnisharp/package/nuget/Microsoft.Extensions.Primitives/1.0.0-rc2-16010</d:GalleryDetailsUrl>
          <d:IconUrl m:null="true" />
          <d:IsLatestVersion m:type="Edm.Boolean">false</d:IsLatestVersion>
          <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
          <d:LastEdited m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:LastEdited>
          <d:Published m:type="Edm.DateTime">2016-01-22T20:46:59.9523998</d:Published>
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
    </entry>
    <entry>
        <id>http://proget/nuget/Default/Packages(Id='Algolia.Search',Version='3.6.4')</id>
        <title type="text">Algolia.Search</title>
        <summary type="text"></summary>
        <updated>2017-06-06T20:15:27Z</updated>
        <author><name>Algolia</name></author>
        <link rel="edit-media" title="Package" href="Packages(Id='Algolia.Search',Version='3.6.4')/$value" />
        <link rel="edit" title="Package" href="Packages(Id='Algolia.Search',Version='3.6.4')" />
        <category term="NuGet.Server.DataServices.Package" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme" />
        <content type="application/zip" src="http://proget/nuget/Default/package/Algolia.Search/3.6.4" />
        <m:properties xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices" xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata">
          <d:Version>3.6.4</d:Version>
          <d:Title>Algolia Search API Client for C#</d:Title>
          <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
          <d:Description>Algolia Search API Client for C#</d:Description>
          <d:ReleaseNotes>Changed to single project to support .NET 4.0, .NET 4.5, ASP.NET vNext 1.0, Mono 4.5, Windows 8, Windows 8.1, Windows Phone 8.1, Xamarin iOS, and Xamarin Android.</d:ReleaseNotes>
          <d:Summary></d:Summary>
          <d:ProjectUrl>https://github.com/algolia/algoliasearch-client-csharp</d:ProjectUrl>
          <d:IconUrl>http://www.algolia.com/download/logo.png</d:IconUrl>
          <d:LicenseUrl>https://github.com/algolia/algoliasearch-client-csharp/blob/master/LICENSE.TXT</d:LicenseUrl>
          <d:Copyright>Copyright 2015</d:Copyright>
          <d:Tags>Search Engine Service Instant Typo-Tolerance Realtime</d:Tags>
          <d:Dependencies>Microsoft.Bcl.Async:1.0.168|Microsoft.Net.Http:2.2.29|Newtonsoft.Json:6.0.8|PCLCrypto:1.0.1.15115</d:Dependencies>
          <d:IsLocalPackage m:type="Edm.Boolean">true</d:IsLocalPackage>
          <d:Created m:type="Edm.DateTime">2017-06-06T20:15:27.0800000Z</d:Created>
          <d:Published m:type="Edm.DateTime">2017-06-06T20:15:27.0800000Z</d:Published>
          <d:PackageSize m:type="Edm.Int64">38807</d:PackageSize>
          <d:PackageHash>5eypGYXO4aHaXsSSgkFu0GkB5ijqqYbSW+l5yBnFxHinvhZl4DNBk1AUMSrFvnqD1aTAtRF4qYbW8MoXqcr+kA==</d:PackageHash>
          <d:IsLatestVersion m:type="Edm.Boolean">true</d:IsLatestVersion>
          <d:IsAbsoluteLatestVersion m:type="Edm.Boolean">true</d:IsAbsoluteLatestVersion>
          <d:IsProGetHosted m:type="Edm.Boolean">true</d:IsProGetHosted>
          <d:IsPrerelease m:type="Edm.Boolean">false</d:IsPrerelease>
          <d:IsCached m:type="Edm.Boolean">false</d:IsCached>
          <d:NormalizedVersion>3.6.4</d:NormalizedVersion>
          <d:Listed m:type="Edm.Boolean">true</d:Listed>
          <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
          <d:HasSymbols m:type="Edm.Boolean">false</d:HasSymbols>
          <d:HasSource m:type="Edm.Boolean">false</d:HasSource>
          <d:DownloadCount m:type="Edm.Int32">0</d:DownloadCount>
          <d:VersionDownloadCount m:type="Edm.Int32">0</d:VersionDownloadCount>
        </m:properties>
    </entry>
    <entry>
        <id>https://api.bintray.com/nuget/fint/nuget/Packages(Id='fint-eventsource',Version='0.4.0.1')</id>
        <title type="text">fint-eventsource</title>
        <summary type="text">An eventsource(Server-Sent Events client) implementation for .Net.</summary>
        <updated>2017-05-04T11:03:27Z</updated>
        <author><name>erizet</name></author>
        <link rel="edit" title="V2FeedPackage" href="Packages(Id='fint-eventsource',Version='0.4.0.1')"/>
        <link rel="self" title="V2FeedPackage" href="Packages(Id='fint-eventsource',Version='0.4.0.1')"/>
        <category term="NuGetGallery.OData.V2FeedPackage" scheme="http://schemas.microsoft.com/ado/2007/08/dataservices/scheme"/>
        <content type="application/zip" src="https://api.bintray.com/nuget/fint/nuget/Download/fint-eventsource/0.4.0.1"/>
        <m:properties xmlns:m="http://schemas.microsoft.com/ado/2007/08/dataservices/metadata" xmlns:d="http://schemas.microsoft.com/ado/2007/08/dataservices">
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
          <d:Language m:null="true"/>
          <d:Published m:type="Edm.DateTime">2017-05-04T11:03:28</d:Published>
          <d:PackageHash>otpBPpuwCOPT5J12azb9MvStj2+WA1nqX/8aAkNjO7Wuohsg/M+d17l1M6k9D4c+B4k6/3XC376eMmbb7TG68A==</d:PackageHash>
          <d:PackageHashAlgorithm>SHA512</d:PackageHashAlgorithm>
          <d:PackageSize m:type="Edm.Int64">9989</d:PackageSize>
          <d:ProjectUrl>https://github.com/fintprosjektet</d:ProjectUrl>
          <d:ReleaseNotes></d:ReleaseNotes>
          <d:RequireLicenseAcceptance m:type="Edm.Boolean">false</d:RequireLicenseAcceptance>
          <d:Tags>fint sse server-sent</d:Tags>
          <d:Title>EventSource4Net</d:Title>
          <d:VersionDownloadCount m:type="Edm.Int32">0</d:VersionDownloadCount>
          <d:Authors>erizet</d:Authors>
          <d:MinClientVersion m:null="true"/>
        </m:properties>
    </entry>
</feed>"##;

        let feed_1: Feed = serde_xml_rs::from_reader(feed_1_serialized.as_bytes()).unwrap();
        let feed_2: Feed = serde_xml_rs::from_reader(feed_2_serialized.as_bytes()).unwrap();

        let feed_1_only_packages = feed_1
            .subtract(&feed_2)
            .into_iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<String>>()
            .join("\n");
        let feed_2_only_packages = feed_2
            .subtract(&feed_1)
            .into_iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<String>>()
            .join("\n");

        assert_eq!(feed_1_only_packages, "_51Wp.AccountSdk 1.0.0");
        assert_eq!(feed_2_only_packages, "fint-eventsource 0.4.0.1");
    }
}
