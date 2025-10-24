use iced::widget::{pick_list, button, column, row, text, Column};
use iced::Theme;
use std::process::exit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum File {
    New,
    Open,
    Save,
    SaveAs,
    Close,
    Exit,
    None,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::New => "New",
            Self::Open => "Open...",
            Self::Save => "Save...",
            Self::SaveAs => "Save As...",
            Self::Close => "Close",
            Self::Exit  => "Exit",
            Self::None  => "File",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Edit {
    Undo,
    Cut,
    Copy,
    Clear,
    Delete,
    SelectAll,
    DeSelectAll,
    None
}

impl std::fmt::Display for Edit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Undo => "Undo",
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Clear => "Clear",
            Self::Delete => "Delete",
            Self::SelectAll => "Select All",
            Self::DeSelectAll => "De-select All",
            Self::None => "Edit",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum View {
    None,
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "View",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Options {
    None,
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "Options",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Help {
    OnHelp,
    OnKeys,
    Index,
    Tutorial,
    OnVersion,
    Warranty,
    Copying,
    None,
}

impl std::fmt::Display for Help {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OnHelp => "On Help...",
            Self::OnKeys => "On Keys...",
            Self::Index => "Index...",
            Self::Tutorial => "Tutorial",
            Self::OnVersion => "On Version",
            Self::Warranty => "Warranty",
            Self::Copying => "Copying",
            Self::None => "Help",
        })
    }
}

struct State {
}

impl Default for State {
    fn default() -> Self {Self{ }}
}

pub fn main() -> iced::Result {
    iced::application("Menubar", update, view)
        .theme(|_| Theme::Light)
        .centered()
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    FileSelected(File),
    EditSelected(Edit),
    ViewSelected(View),
    OptionsSelected(Options),
    HelpSelected(Help),
}

fn update(value: &mut State, message: Message) {
    eprintln!("Message is {:?}",message);
    match message {
        Message::FileSelected(File::Exit) => exit(0),
        _ => (),
    }
}


fn view(state: &State) -> Column<'_, Message> {
    let filemenuitems = [
        File::New,
        File::Open,
        File::Save,
        File::SaveAs,
        File::Close,
        File::Exit,
    ];        
    let editmenuitems = [
        Edit::Undo,
        Edit::Cut,
        Edit::Copy,
        Edit::Clear,
        Edit::Delete,
        Edit::SelectAll,
        Edit::DeSelectAll,
    ];
    let viewmenuitems = [
    ];
    let optionsmenuitems = [
    ];
    let helpmenuitems = [
        Help::OnHelp,
        Help::OnKeys,
        Help::Index,
        Help::Tutorial,
        Help::OnVersion,
        Help::Warranty,
        Help::Copying,
    ];
    column![
        row![pick_list(filemenuitems,
                      Some(File::None),
                      Message::FileSelected,
             ), 
             pick_list(editmenuitems,
                        Some(Edit::None),
                        Message::EditSelected,
             ), 
             pick_list(viewmenuitems,
                        Some(View::None),
                        Message::ViewSelected,
             ), 
             pick_list(optionsmenuitems,
                        Some(Options::None),
                        Message::OptionsSelected,
            
             ), 
             pick_list(helpmenuitems,
                        Some(Help::None),
                        Message::HelpSelected,
             ),
        ],
    ]
}

