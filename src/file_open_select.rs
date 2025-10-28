// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-27 13:53:29
//  Last Modified : <251028.1359>
//
//  Description	
//
//  Notes
//
//  History
//	
/////////////////////////////////////////////////////////////////////////////
//    Copyright (C) 2025  Robert Heller D/B/A Deepwoods Software
//			51 Locke Hill Road
//			Wendell, MA 01379-9728
//
//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.
//
//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program; if not, write to the Free Software
//    Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
// 
//
//////////////////////////////////////////////////////////////////////////////

use std::io;
use std::fs::{canonicalize,self, DirEntry,read_dir};
use std::path::{Path,PathBuf};
use std::fmt;

use fast_glob::glob_match;
use iced::widget::{button, column, row, text, Column, container, scrollable, 
                    pick_list, text_input};
use iced::{window,Background,Theme,Center, Color, Element};
use iced::widget::button::Style;

#[derive(Debug, Clone)] 
pub enum Message {
    UpDir,
    OpenFile,
    Cancel,
    SelectDir(String),
    ContentChanged(String),
    TypeSelected(FileType),
    SelectFile(String),
}

#[derive(Debug, Clone)]
pub enum Action {
    Cancel,
    Open(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct FileType {
    name: String,
    extension: String, 
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.extension)
    }
}    

impl FileType {
    pub fn new(name: String, ext: String) -> Self {
        Self {name: name, extension: ext}
    }
    pub fn GlobMatch(&self,filename: String) -> bool {
        glob_match(self.extension.clone(),filename)            
            
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::new("All Files".to_string(),"*".to_string())
    }
}

#[derive(Debug, Clone)] 
pub struct FileOpenSelect {
    defaultextension: String,
    filetypes: Vec<FileType>,
    initialdir: PathBuf,
    initialfile: PathBuf,
    title: String,
    dirs: Vec<String>,
    currentfile: String,
    files: Vec<String>,    
}



impl FileOpenSelect {
    pub fn Title(&self) -> String {self.title.clone()}
    pub fn new(defaultextension: String,
                filetypes: &[FileType], initialdir: PathBuf,
                initialfile: PathBuf, title: String) -> Self {
        let currentfile_path = initialfile.as_path();
        let currentfile_lossy = currentfile_path.to_string_lossy();
        let currentfile = currentfile_lossy.to_string();
        let mut this = Self {defaultextension: defaultextension,
                             filetypes: Vec::new(),
                             initialdir: match canonicalize(initialdir) {
                                Err(p) => canonicalize(".").ok().unwrap(),
                                Ok(f) => f,
                              },
                              initialfile: initialfile.clone(),
                              title: title, dirs: Vec::new(),
                              currentfile: currentfile,
                              files: Vec::new() };
        for i in 0..filetypes.len() {
            this.filetypes.push(filetypes[i].clone());
        }
        if this.filetypes.len() == 0 {
            this.filetypes.push(FileType::default());
        }
        let temp1 = this.initialdir.clone();
        this.buildDirs(&temp1);
        let temp2 = this.filetypes[0].clone();
        this.populateFiles(&temp2);
        this
    }
    fn buildDirs(&mut self,cur: &PathBuf) {
        let mut current = PathBuf::from(cur);
        self.dirs = vec![current.to_string_lossy().to_string()];
        loop {
            match current.parent() {
                None => {break;},
                Some(parent) => {
                    self.dirs.insert(0,parent.to_string_lossy().to_string());
                    current = PathBuf::from(parent);
                },
            }
        }
        
    }
    pub fn update(&mut self, message: Message) -> Option<Action> {
        eprintln!("*** FileOpenSelect::update() message is {:?}",message);
        match message {
            Message::ContentChanged(content) => {
                self.currentfile = content;
                None
            },
            Message::TypeSelected(filetype) => {
                self.populateFiles(&filetype);
                None
            }
            Message::UpDir => {
                let path = PathBuf::from(self.dirs.last().unwrap());
                let parent = path.parent();
                if parent.is_some() {
                    self.buildDirs(&PathBuf::from(parent.unwrap()));
                    let temp = self.filetypes[0].clone();
                    self.populateFiles(&temp);
                }
                None
            },
            Message::SelectDir(dir) => {
                self.buildDirs(&PathBuf::from(dir));
                let temp = self.filetypes[0].clone();
                self.populateFiles(&temp);
                None
            },
            Message::OpenFile => {
                let path = match canonicalize(self.currentfile.clone()) {
                    Ok(p) => p,
                    Err(e) => {eprintln!("{}",e); return None;}
                };
                if path.is_dir() {
                    self.buildDirs(&path);
                    let temp = self.filetypes[0].clone();
                    self.populateFiles(&temp);
                    None
                } else {
                    Some(Action::Open(path.to_string_lossy().to_string()))
                }
            },
            Message::Cancel => {
                Some(Action::Cancel)
            },
            Message::SelectFile(name) => {
                self.currentfile = name;
                None
            },
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        column![
            row!["Directory:", 
                 self.DirTree(), 
                 button("^").on_press(Message::UpDir)
            ],
            scrollable(self.DirList()).height(160.0).width(450.0),
            row!["File name:",
                 self.CurrentFile(),
                 button("Open").on_press(Message::OpenFile)
            ],
            row!["File types:",
                self.TypeList(),
                button("Cancel").on_press(Message::Cancel)
            ]
        ].into()
    }
    fn DirTree(&self) -> Element<'_, Message> {
        pick_list(self.dirs.clone(),self.dirs.last(),Message::SelectDir).into()
    }
    fn populateFiles(&mut self,filetype: &FileType)
    {
        self.files = Vec::new();
        for entry in fs::read_dir(self.dirs.last().unwrap()).ok().unwrap() {
            let entryPath = entry.ok().unwrap().path();
            if entryPath.is_dir() {
                let ent = entryPath.to_string_lossy().to_string() + "/";
                self.files.push(ent);
            } else {
                let fname = entryPath.file_name().unwrap().to_string_lossy();
                if filetype.GlobMatch(fname.to_string()) {
                    self.files.push(entryPath.to_string_lossy().to_string());
                }
            }
        }
        self.files.sort_unstable();
    }
    fn white(_:&Theme, _:button::Status) -> Style {
        Style {background:Some(Background::Color(Color::from_rgb8(255,255,255))),
                                     ..Style::default()}
    } 
    fn DirList(&self) -> Element<'_, Message> {
        column(self.files.iter()
                    .map(|name| button(name.as_str())
                                .style(Self::white)
                                .on_press(Message::SelectFile(name.to_string()))
                                .into())).into()
    }
    fn CurrentFile(&self) -> Element<'_, Message> {
        text_input("",&self.currentfile)
            .on_input(Message::ContentChanged)
            .into()
    }
    fn TypeList(&self) -> Element<'_, Message> {
        pick_list(self.filetypes.clone(),
                    Some(self.filetypes[0].clone()),
                    Message::TypeSelected).into()
    }
    pub fn Settings() -> window::Settings {
        window::Settings {
            size: (450.0, 250.0).into(),
            ..window::Settings::default()
        }
    }
    pub fn Setup(defext: &String,initdir: &String,initfile: &String,title: &String) -> Self {
        Self::new(defext.to_string(),&[],initdir.into(),initfile.into(),title.to_string() )
    }
}

impl Default for FileOpenSelect {
    fn default() -> Self {
        FileOpenSelect::new("".to_string(),&[],PathBuf::from("."),
                            PathBuf::from(""),"".to_string())
    }
}

pub type Manager = FileOpenSelect;
