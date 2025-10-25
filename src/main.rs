// -!- rust -!- //////////////////////////////////////////////////////////////
//
//  System        : 
//  Module        : 
//  Object Name   : $RCSfile$
//  Revision      : $Revision$
//  Date          : $Date$
//  Author        : $Author$
//  Created By    : Robert Heller
//  Created       : 2025-10-24 10:24:21
//  Last Modified : <251025.0742>
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

#![allow(unused_imports)]
#![doc = include_str!("../README.md")]


mod menubar;
use crate::menubar::*;
use iced::widget::{pane_grid, pick_list, button, column, row, text, Column,
                   scrollable, responsive, container, image}; 
use iced::widget::pane_grid::{Axis,PaneGrid};
use iced::{Background,Theme,Center, Color, Element, Fill, Size, Subscription};
use iced::widget::button::Style;
use iced::keyboard;
use std::process::exit;

#[derive(Debug, Clone, Copy)]
enum Message {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
    FileSelected(File),
    EditSelected(Edit),
    ViewSelected(View),
    OptionsSelected(Options),
    TrainsSelected(Trains),
    StationsSelected(Stations),
    CabsSelected(Cabs),
    NotesSelected(Notes),
    HelpSelected(Help),
}

struct TimeTableGUI {
    panes: pane_grid::State<Pane>,
}

impl TimeTableGUI {
    fn new() -> Self {
        let (mut panes, main) = pane_grid::State::new(Pane::Main);
        let Some((buttonpane,split)) = panes.split(Axis::Vertical,main,Pane::Buttons) else {panic!("failed to create buttons");};
        panes.resize(split,0.7490561);
        TimeTableGUI {
            panes, 
        }
    }
    fn update(&mut self, message: Message) {
        eprintln!("*** TimeTableGUI::update(): message is {:?}",message);
        match message {
            Message::FileSelected(File::Exit) => exit(0),
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                eprintln!("*** TimeTableGUI::update(): split is {:?}",split);
                self.panes.resize(split, ratio);
            }
            _ => (),
        }
    }

    fn white(_:&Theme, _:button::Status) -> Style {
        Style {background:Some(Background::Color(Color::from_rgb8(255,255,255))),
                                     ..Style::default()}
    } 
    fn view(&self) -> Element<'_, Message> {

        let pane_grid = pane_grid(&self.panes, |pane, state, is_maximized| {
                match state {
                    Pane::Main => 
                        pane_grid::Content::new(text("This is the Main pane")),
                    Pane::Buttons => 
                        pane_grid::Content::new(text("This is the Buttons pane")),
                }
            })
            .spacing(10)
            .on_click(Message::Clicked)
            .on_drag(Message::Dragged)
            .on_resize(10, Message::Resized);
        
        let filemenuitems = [
            File::New,
            File::Open,
            File::Save,
            File::SaveAs,
            File::Print,
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
            View::ViewOneTrain,
            View::ViewAllTrains,
            View::ViewOneStation,
            View::ViewAllStations,
            View::ViewOneNote,
            View::ViewAllNotes,
        ];
        let optionsmenuitems = [
            Options::EditSystemConfiguration,
            Options::SaveSystemConfiguration,
            Options::ReloadSystemConfiguration,
        ];
        let trainsmenuitems = [
            Trains::AddTrain,
            Trains::EditTrain,
            Trains::DeleteTrain,
        ];
        let stationsmenuitems = [
            Stations::SetDuplicateStation,
            Stations::ClearDuplicateStation,
            Stations::AddStorageTrack,
        ];
        let cabsmenuitems = [
            Cabs::AddACab,
        ];
        let notesmenuitems = [
            Notes::CreateNewNote,
            Notes::EditExistingNote,
            Notes::AddNoteToTrain,
            Notes::AddNoteToTrainAtStation,
            Notes::RemoveNoteFromTrain,
            Notes::RemoveNoteFromTrainAtStation
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
                 ).text_size(10), 
                 pick_list(editmenuitems,
                            Some(Edit::None),
                            Message::EditSelected,
                 ).text_size(10), 
                 pick_list(viewmenuitems,
                            Some(View::None),
                            Message::ViewSelected,
                 ).text_size(10), 
                 pick_list(optionsmenuitems,
                            Some(Options::None),
                            Message::OptionsSelected,
                 ).text_size(10), 
                 pick_list(trainsmenuitems,
                            Some(Trains::None),
                            Message::TrainsSelected,
                 ).text_size(10), 
                 pick_list(stationsmenuitems,
                            Some(Stations::None),
                            Message::StationsSelected,
                 ).text_size(10), 
                 pick_list(cabsmenuitems,
                            Some(Cabs::None),
                            Message::CabsSelected,
                 ).text_size(10), 
                 pick_list(notesmenuitems,
                            Some(Notes::None),
                            Message::NotesSelected,
                 ).text_size(10), 
                 pick_list(helpmenuitems,
                            Some(Help::None),
                            Message::HelpSelected,
                 ).text_size(10),
            ],
            row![
                button(image("images/new.png"))
                        .style(Self::white)
                        .on_press(Message::FileSelected(File::New)),
                button(image("images/open.png"))
                        .style(Self::white)
                        .on_press(Message::FileSelected(File::Open)),
                button(image("images/save.png"))
                        .style(Self::white)
                        .on_press(Message::FileSelected(File::Save)),
                button(image("images/addtrain.png"))
                        .style(Self::white)
                        .on_press(Message::TrainsSelected(Trains::AddTrain)),
                button(image("images/deletetrain.png"))
                        .style(Self::white)
                        .on_press(Message::TrainsSelected(Trains::DeleteTrain)),
                button(image("images/setdupstation.png"))
                        .style(Self::white)
                        .on_press(Message::StationsSelected(Stations::SetDuplicateStation)),
                button(image("images/cleardupstation.png"))
                        .style(Self::white)
                        .on_press(Message::StationsSelected(Stations::ClearDuplicateStation)),
                button(image("images/addstorage.png"))
                        .style(Self::white)
                        .on_press(Message::StationsSelected(Stations::AddStorageTrack)),
                button(image("images/addcab.png"))
                        .style(Self::white)
                        .on_press(Message::CabsSelected(Cabs::AddACab)),
                button(image("images/createnote.png"))
                        .style(Self::white)
                        .on_press(Message::NotesSelected(Notes::CreateNewNote)),
                button(image("images/editnote.png"))
                        .style(Self::white)
                        .on_press(Message::NotesSelected(Notes::EditExistingNote)),
                button(image("images/addnotetotrain.png"))
                        .style(Self::white)
                        .on_press(Message::NotesSelected(Notes::AddNoteToTrain)),
                button(image("images/addnotetotrainatstation.png"))
                        .style(Self::white)
                        .on_press(Message::NotesSelected(Notes::AddNoteToTrainAtStation)),
                button(image("images/removenotefromtrain.png"))
                        .style(Self::white)
                        .on_press(Message::NotesSelected(Notes::RemoveNoteFromTrain)),
                button(image("images/removenotefromtrainatstation.png"))
                        .style(Self::white)
                        .on_press(Message::NotesSelected(Notes::RemoveNoteFromTrainAtStation)),
                button(image("images/print.png"))
                        .style(Self::white)
                        .on_press(Message::FileSelected(File::Print)),
                button(image("images/close.png"))
                        .style(Self::white)
                        .on_press(Message::FileSelected(File::Exit)),
            ],
             container(pane_grid)
                .width(Fill)
                .height(Fill)
                .padding(0),
        ].into()
    }
}

impl Default for TimeTableGUI {
    fn default() -> Self {
        TimeTableGUI::new()
    }
}

#[derive(Clone, Copy)]
enum Pane {
    Main,
    Buttons,
}


pub fn main() -> iced::Result {
    iced::application("Time Table 2", TimeTableGUI::update, TimeTableGUI::view)
        .theme(|_| Theme::Light)
        .centered()
        .run()
}
