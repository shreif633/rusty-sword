// let guild_members = GuildMembers { 
//     unknown: vec![36, 2, 0, 0], 
//     guild_name: "KalSaga".to_string(), 
//     leader_position_name: "Leader".to_string(), 
//     subleader_position_name: "SubLeader".to_string(), 
//     manager_position_name: "Manager".to_string(), 
//     chief_position_name: "Chief".to_string(), 
//     regular_member_position_name: "Member".to_string(), 
//     temporary_member_position_name: "TempMember".to_string(), 
//     members: vec![
//         Member { name: "Mortaro".to_string(), position: Position::Leader, level: 0 }, 
//         Member { name: "Hermit".to_string(), position: Position::RegularMember, level: 60 }, 
//         Member { name: "CJB".to_string(), position: Position::TemporaryMember, level: 0 }, 
//         Member { name: "Comma".to_string(), position: Position::SubLeader, level: 0 }
//     ] 
// };
// player.send(&mut (&guild_members).into()).await;