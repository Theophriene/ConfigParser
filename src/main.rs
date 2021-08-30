#[macro_use]
extern crate serde_derive;
extern crate quick_xml;

mod config;

fn main()
{
    let t   = std::fs::read_to_string("./test.xml").unwrap();
    let res = config::from_str(&t).unwrap();

    println!("");
    for domain in res
    {
        println!("Domain {}", domain.attrs.get("name").unwrap_or(&String::from("")));
        for group in domain.groups
        {
            println!("\\ Group {}", group.attrs.get("name").unwrap_or(&String::from("")));
            for user in group.users
            {
                println!("   \\ User (id:{}, name:{})", user.attrs.get("id").unwrap_or(&String::from("")), user.attrs.get("name").unwrap_or(&String::from("")));
            }
        }
    }
}
