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
//  Last Modified : <251027.2107>
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
use std::path::PathBuf;

use iced::widget::{button, column, row, text, Column, container, scrollable, 
                    pick_list, text_input};
use iced::{window,Background,Theme,Center, Color, Element};

#[derive(Debug, Clone)] 
pub enum Message {
    UpDir,
    OpenFile,
    Cancel,
    SelectDir(String),
    SelectFileType,
}

#[derive(Debug, Clone)]
pub enum Action {
    Cancel,
    Open(String),
}

#[derive(Debug, Clone)]
pub struct FileType {
    name: String,
    extension: String, 
    mactype: Option<String>,
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
}



impl FileOpenSelect {
    pub fn Title(&self) -> String {self.title.clone()}
    pub fn new(defaultextension: String,
                filetypes: &[FileType], initialdir: PathBuf,
                initialfile: PathBuf, title: String) -> Self {
        
        let mut this = Self {defaultextension: defaultextension,
                             filetypes: Vec::new(),
                             initialdir: match canonicalize(initialdir) {
                                Err(p) => canonicalize(".").ok().unwrap(),
                                Ok(f) => f,
                              },
                              initialfile: initialfile,
                              title: title, dirs: Vec::new(),
                              currentfile: String::new() };
        for i in 0..filetypes.len() {
            this.filetypes.push(filetypes[i].clone());
        }
        let mut current = this.initialdir.clone();
        this.dirs = vec![current.to_string_lossy().to_string()];
        loop {
            match current.parent() {
                None => {break;},
                Some(parent) => {
                    this.dirs.insert(0,parent.to_string_lossy().to_string());
                    current = PathBuf::from(parent);
                },
            }
        }
        this
    }
    pub fn update(&mut self, message: Message) -> Option<Action> {
        match message {
            _ => None,
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        column![
            row!["Directory:", 
                 self.DirTree(), 
                 button("^").on_press(Message::UpDir)
            ],
            scrollable(self.DirList()),
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
    fn DirList(&self) -> Element<'_, Message> {
        let mut files: Vec<String> = Vec::new();
        for entry in fs::read_dir(self.dirs.last().unwrap()).ok().unwrap() {
            let ent = entry.ok().unwrap().path().to_string_lossy().to_string();
            files.push(ent);
        }
        column(files.iter().map(|name| text!("{name}").into())).into()
    }
    fn CurrentFile(&self) -> Element<'_, Message> {
        text_input("",&self.currentfile).into()
    }
    fn TypeList(&self) -> Element<'_, Message> {
        text!("not implemented").into()
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
