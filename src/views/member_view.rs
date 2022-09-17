use crate::models::member::Member;

pub trait MemberDisplay {
    fn display_member_info(member: Member);
    fn ls_simple(members: Vec<Member>);
    fn ls_verbose(members: Vec<Member>);
    fn get_member_info() -> Member;
    fn edit_member_info(member: Member) -> Member;
}

pub struct MemberView {}
