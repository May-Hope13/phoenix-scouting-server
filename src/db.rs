use sled::Db;

pub trait PssDb {
    fn district_events(&self, district: impl Into<String>) -> impl Iterator<Item = String>;
}


impl PssDb for Db {
    fn district_events(&self, district: impl Into<String>) -> impl Iterator<Item = String> {
        let tree = self.open_tree("district_events").unwrap();

        let district = district.into();

        tree.scan_prefix(district.clone()).into_iter().filter_map(move |ev| {
            if let Ok((k, _)) = ev {
                let key = String::from_utf8(k.to_vec()).unwrap();
                let ev = key.strip_prefix(&format!("{district}_")).unwrap().to_string();
                Some(ev.clone())
            } else {
                error!("some weird bug");
                None
            }
        }).into_iter()
    }
}
