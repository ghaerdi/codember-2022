const USERNAME_FIELD: &str = "usr:";
const REQUIRED_FIELDS: [&str; 6] = [USERNAME_FIELD, "eme:", "psw:", "loc:", "age:", "fll:"];

async fn fetch_users() -> reqwest::Result<Vec<String>> {
    let file = reqwest::get("https://codember.dev/users.txt")
        .await?
        .text()
        .await?
        .split("\n\n")
        .map(|user| user.to_string())
        .collect::<Vec<String>>();
    Ok(file)
}

struct Users {
    valid_users: Vec<String>,
}

impl Users {
    /// Receive a list of users but it save in struct just the valid users
    fn new(users: Vec<String>) -> Self {
        Self {
            valid_users: Self::filter_valid_users(users),
        }
    }

    fn filter_valid_users(users: Vec<String>) -> Vec<String> {
        let is_valid_user = |user: String| {
            REQUIRED_FIELDS
                .into_iter()
                .map(|keys| user.contains(keys))
                .all(|v| v)
        };

        users
            .into_iter()
            .filter(|user| is_valid_user(user.to_string()))
            .collect::<Vec<String>>()
    }

    /// Get the lenght of valid users
    fn len(&self) -> usize {
        self.valid_users.len()
    }

    /// Get the last valid user
    fn last(&self) -> String {
        self.valid_users.last().unwrap().to_string()
    }

    /// Extract the username of a user
    fn extract_username(user: String) -> String {
        let temp = user
            .split(' ')
            .find(|text| text.contains(USERNAME_FIELD))
            .unwrap();

        let index = temp.find('@').unwrap();
        let result = &temp[index..];
        result.to_string()
    }
}

pub async fn result() -> reqwest::Result<()> {
    let users = Users::new(fetch_users().await?);
    let last_user_username = Users::extract_username(users.last());

    println!("challenge 01");
    println!("{}{}", users.len(), last_user_username);
    Ok(())
}
