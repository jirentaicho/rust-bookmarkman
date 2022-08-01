use std::io;

enum Command {
    CONTINUE,
    EXIT,
    //追加
    PRINT,
}

#[derive(Debug)]
struct Bookmark {
    url : String,
    title: String,
}

fn get_command(input : Option<i8>) -> Result<Command,String> {
    match input {
        Some(1) => Ok(Command::CONTINUE),
        Some(2) => Ok(Command::EXIT),
        Some(3) => Ok(Command::PRINT),
        Some(_) => Err("入力されたコマンドが存在しません".to_string()),
        None => Err("システム例外".to_string()),
    }
}

fn register_bookmark(bookmarks : &mut Vec<Bookmark>) {

    println!("URLを入力してください");
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();

    println!("タイトルを入力してください");
    let mut title = String::new();
    io::stdin().read_line(&mut title).unwrap();

    let bookmark = Bookmark {
        url,
        title,
    };

    bookmarks.push(bookmark);
}

fn print_bookmark(bookmarks : &Vec<Bookmark>){
    println!("ブックマーク一覧");
    for bookmark in bookmarks {
        println!("{:?}", bookmark);
    }
}

fn main() {

    let mut bookmarks : Vec<Bookmark> = Vec::new();

    loop{ 
        println!("コマンドを入力してください");
        println!("1 = ブックマークを登録する");
        println!("2 = アプリケーションを終了する");
        println!("3 = ブックマークを出力する");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command_num : Option<i8> = match input.trim().parse() {
            Ok(num) => Some(num),
            Err(_) => {
                println!("存在しないコマンドが入力されました");
                break
            },
        };

        let command_enum = get_command(command_num);

        match command_enum.unwrap() {
            Command::CONTINUE => register_bookmark(&mut bookmarks), 
            Command::PRINT => print_bookmark(&bookmarks),
            Command::EXIT => {
                println!("アプリケーションを終了します");
                break
            }
        }

    }

}
