#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Permission(pub String);

impl Permission {
    /// foo.bar.bazのようなパスから[*, foo.*, foo.bar.*, foo.bar.baz]なパスの配列を求める
    pub fn get_parent_paths(path: &String) -> Vec<String> {
        let splited = {
            let mut splited = path.split(".").collect::<Vec<&str>>();
            splited.pop();
            splited
        };
        let splited = splited;
        let mut result = splited.into_iter().fold(vec![String::from("*")], |accumrator, current| {
            let last = accumrator.last().unwrap().clone().to_string();
            let last_splited: Vec<&str> = last.split(".").collect();
            // [*] || ["foo.*"]
            let mut last_splited = last_splited.iter().map(|str| str.to_string()).collect::<Vec<String>>();
            // [*, "foo.*"] || [*, "foo.*", "foo.bar.*"]
            last_splited.insert(last_splited.len() - 1, String::from(current));
            [&accumrator[..], &vec![last_splited.join(".")][..]].concat()
        });
        result.push(path.clone());
        result.into_iter().fold(Vec::<String>::new(), |mut accumurator, current| {
            if !accumurator.contains(&current) {
                accumurator.push(current);
            }
            accumurator
        })
    }

    /// 子を取得する
    pub fn get_parents(&self) -> Vec<Permission> {
        Self::get_parent_paths(&self.0)
            .into_iter()
            .map(|path| Permission(path))
            .collect::<Vec<Permission>>()
    }
}

impl From<&str> for Permission {
    fn from(path: &str) -> Permission {
        Permission(path.to_string())
    }
}

/// 権限が含まれているかを返す
pub trait Includes<T> {
    fn includes(&self, required: T) -> bool;
}

impl Includes<&str> for Permission {
    fn includes(&self, required: &str) -> bool {
        Self::from(required).get_parents().contains(&self)
    }
}

impl Includes<&String> for Permission {
    fn includes(&self, required: &String) -> bool {
        Self(required.clone()).get_parents().contains(&self)
    }
}

impl Includes<&Vec<String>> for Permission {
    fn includes(&self, required: &Vec<String>) -> bool {
        required.into_iter().all(
            |required| self.includes(required)
        )
    }
}

#[test]
fn test_get_children() {
    let p1 = Permission::from("foo.bar.baz");
    assert_eq!(
        p1.get_parents(),
        ["*", "foo.*", "foo.bar.*", "foo.bar.baz"].iter().map(|path| Permission::from(*path)).collect::<Vec<Permission>>()
    );

    let p2 = Permission::from("xxx.yyy.zzz.*");
    assert_eq!(
        p2.get_parents(),
        ["*", "xxx.*", "xxx.yyy.*", "xxx.yyy.zzz.*"].iter().map(|path| Permission::from(*path)).collect::<Vec<Permission>>()
    );
}

#[test]
fn test_includes() {
    let permission = Permission::from("foo.*");
    assert!(permission.includes("foo.bar"));
    assert!(permission.includes("foo.bar.*"));
    assert!(!permission.includes("xxx"));
}
