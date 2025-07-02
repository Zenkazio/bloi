// use std::path::PathBuf;

// pub fn store_routine(target_path: &PathBuf, user_choice: &mut UserChoice) -> Result<(), String> {
//     let store_path: PathBuf = get_default_store_path()?.join(absolute_to_relative(target_path));

//     dbg!(target_path);
//     dbg!(store_path);

//     return Ok(());
//     let mut target_state = classify_path(target_path);
//     let mut store_state = classify_path(&store_path);
//     // dbg!(target_path);
//     // dbg!(&path_to_store);
//     loop {
//         match (target_state, store_state) {
//             (_, PathType::SymLink) => {
//                 return Err(format!(
//                     "serious problem symlink in store doesn't make sense\n{:?}",
//                     store_path
//                 ));
//             }
//             (PathType::SymLink, PathType::File) => {} //nothing to do here this is the wanted state for storing
//             (PathType::SymLink, _) => {
//                 return Err(format!(
//                     "this is a hard problem symlink exists but nothing in store serious data loss possible\n{:?}",
//                     target_path
//                 ));
//             }
//             (PathType::File, PathType::File) => {
//                 match *user_choice {
//                     UserChoice::NoChoice | UserChoice::TakeStore | UserChoice::TakeTarget => {
//                         println!("{:?}", target_path);
//                         get_user_choice(user_choice)?;
//                     }
//                     _ => {} //skip it
//                 }
//                 match *user_choice {
//                     UserChoice::TakeStore | UserChoice::TakeStoreAll => {
//                         mv!(fs::remove_file(target_path));
//                         target_state = PathType::NoExist;
//                         store_state = PathType::File;
//                         continue;
//                     }
//                     UserChoice::TakeTarget | UserChoice::TakeTargetAll => {
//                         mv!(fs::remove_file(&store_path));
//                         target_state = PathType::File;
//                         store_state = PathType::NoExist;
//                         continue;
//                     }
//                     _ => {
//                         return Err(format!("this should not be possible"));
//                     }
//                 }

//                 //return Err(format!("conflict both are files\n{:?}", target_path));
//             } //conflict which user needs to resolve
//             (PathType::File, PathType::NoExist) => {
//                 mv!(fs::create_dir_all(store_path.parent().unwrap()));
//                 fs::rename(target_path, &store_path));
//                 symlink(&store_path, target_path));
//             } // this is the case for storing
//             (PathType::NoExist, PathType::NoExist) => {
//                 return Err(format!(
//                     "nothing in target and store mabye old path in config\n{:?}",
//                     target_path
//                 ));
//             }
//             (PathType::Dir, PathType::NoExist) => {
//                 fs::create_dir_all(&store_path));
//                 for entry in fs::read_dir(target_path)) {
//                     store_routine(&entry).path(), user_choice)?;
//                 }
//             }
//             (PathType::NoExist, PathType::File) => {
//                 fs::create_dir_all(target_path.parent().unwrap()));
//                 symlink(&store_path, target_path));
//             } // this is the case if no target exists just create symlink
//             (PathType::Dir, PathType::File) => {
//                 return Err(format!(
//                     "serious problem dir != file mismatch\n{:?}",
//                     target_path
//                 ));
//             }
//             (PathType::File, PathType::Dir) => {
//                 return Err(format!(
//                     "serious problem dir != file mismatch\n{:?}",
//                     target_path
//                 ));
//             }
//             (PathType::Dir, PathType::Dir) => {
//                 for entry in fs::read_dir(target_path)) {
//                     //dbg!(&entry).path());
//                     store_routine(&entry).path(), user_choice)?;
//                 }
//                 // println!(
//                 //     "this path (dir - dir) is not fully developed mabye you have to delete something yourself"
//                 // );
//                 // println!("{:?}", target_path);
//             } //this is already good I guess ~later~ I was wrong...
//             (PathType::NoExist, PathType::Dir) => {
//                 copy_dir_all(&store_path, target_path)?; //dangerous
//                 delete_all(&store_path)?; //really dangerous
//                 store_routine(target_path, user_choice)?;
//                 //this is stupidly dangerous in case I wrote the copy wrong somewhere
//             }
//         }
//         break;
//     }
//     Ok(())
// }

// fn absolute_to_relative(absolute_path: &PathBuf) -> PathBuf {
//     //strip_prefix could be better :/
//     let mut temp = absolute_path.to_str().unwrap().to_string();
//     temp.remove(0);
//     PathBuf::from(temp)
// }

// fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
//     if !dst.exists() {
//         mv!(fs::create_dir_all(dst));
//     }
//     for entry in mv!(fs::read_dir(src)) {
//         let path = mv!(entry).path();
//         let rel_path = path.strip_prefix(src).unwrap();
//         let dest_path = dst.join(rel_path);

//         if path.is_file() {
//             mv!(fs::copy(path, dest_path));
//         } else if path.is_dir() {
//             copy_dir_all(&path, &dest_path)?;
//         }
//     }
//     Ok(())
// }

// fn delete_all(path: &PathBuf) -> Result<(), String> {
//     mv!(fs::remove_dir_all(path));
//     Ok(())
// }

// fn get_user_choice(user_choice: &mut UserChoice) -> Result<(), String> {
//     let mut input = String::new();
//     println!("there are two files make choice");
//     println!("1:take store(default)");
//     println!("2:take target");
//     println!("3:take store for all of this entry");
//     println!("4:take target for all of this entry");

//     mv!(stdin().read_line(&mut input));

//     match input.trim() {
//         "" | "1" => *user_choice = UserChoice::TakeStore,
//         "2" => *user_choice = UserChoice::TakeTarget,
//         "3" => *user_choice = UserChoice::TakeStoreAll,
//         "4" => *user_choice = UserChoice::TakeTargetAll,
//         _ => {
//             return Err(format!("this is very wrong"));
//         }
//     }
//     Ok(())
// }
// #[cfg(test)]
// #[test]
// fn test_store_routine() {
//     let path = PathBuf::from("/home/zenkazio/.config/eww/");
//     store_routine(&path, &mut UserChoice::NoChoice);
// }
