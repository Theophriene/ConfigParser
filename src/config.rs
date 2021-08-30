use std::collections::*;

mod directory;

pub struct Domain
{
    pub attrs     : HashMap<String,String>,
    pub params    : HashMap<String,String>,
    pub variables : HashMap<String,String>,
    pub groups    : Vec<Group>,
}

pub struct Group
{
    pub attrs     : HashMap<String,String>,
    pub params    : HashMap<String,String>,
    pub variables : HashMap<String,String>,
    pub users     : Vec<User>
}

pub struct User
{
    pub attrs     : HashMap<String,String>,
    pub params    : HashMap<String,String>,
    pub variables : HashMap<String,String>,
}




impl From<directory::User> for User 
{
    fn from(given: directory::User) -> Self 
    {
        Self
        {
            attrs     : given.attrs,
            params    : given.params.unwrap_or_default().into(),
            variables : given.variables.unwrap_or_default().into()
        }    
    }
}

impl From<directory::Group> for Group
{
    fn from(given: directory::Group) -> Self
    {
        Self
        {
            attrs     : given.attrs,
            params    : given.params.unwrap_or_default().into(),
            variables : given.variables.unwrap_or_default().into(),
            users     : given.users.unwrap_or_default().user.into_iter().map(User::from).collect()
        }    
    }
}

impl From<directory::Domain> for Domain
{
    fn from(mut given: directory::Domain) -> Self 
    {
        if let Some(users) = given.users
        {
            let mut groups = given.groups.unwrap_or_default();

            groups.group.push(directory::Group 
            {
                attrs     : vec![("name".to_string(), "__groupless__".to_string())].into_iter().collect(),
                params    : None,
                variables : None,
                users     : Some(users)
            });

            given.groups = Some(groups);
        }

        Self 
        {
            attrs     : given.attrs,
            params    : given.params.unwrap_or_default().into(),
            variables : given.variables.unwrap_or_default().into(),
            groups    : given.groups.unwrap_or_default().group.into_iter().map(Group::from).collect()
        }
    }
}




pub fn from_str(s : &str) -> Result<Vec<Domain>,quick_xml::DeError>
{   
    Ok(quick_xml::de::from_str::<Vec<directory::Domain>>(s)?.into_iter().map(Domain::from).collect())
}