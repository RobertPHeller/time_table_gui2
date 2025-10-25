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
//  Last Modified : <251025.1714>
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
use time_table::*;
use iced::widget::{pane_grid, pick_list, button, column, row, text, Column,
                   scrollable, responsive, container, image, canvas}; 
use iced::widget::pane_grid::{Axis,PaneGrid};
use iced::{Background,Theme,Center, Color, Element, Fill, Size, Subscription,
            Renderer, Rectangle, mouse, Point};
use iced::widget::button::Style;
use iced::widget::canvas::{Text,Path};
use iced::keyboard;
use std::process::exit;
use std::collections::HashMap;

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
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum StationArrayIndex {
    Y(usize),
    Smile(usize),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum StorageArrayIndex {
    Y(String,String),
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum CabArrayIndex {
    Y(String),
}

#[derive(Debug, Clone)]
pub struct ChartDisplay {
    lheight: f64,
    topofcabs: f64,
    cabheight: f64,
    bottomofcabs: f64,
    numberofcabs: usize,
    cabarray: HashMap<CabArrayIndex,f64>,
    topofchart: f64,
    chartheight: f64,
    bottomofchart: f64,
    totallength: f64,
    chartstationoffset: f64,
    topofstorage: f64,
    storagetrackheight: f64,
    bottomofstorage: f64,
    numberofstoragetracks: usize,
    storageoffset: f64,
    stationarray: HashMap<StationArrayIndex,f64>, 
    storagearray: HashMap<StorageArrayIndex,f64>,

    timescale: u32,
    timeinterval: u32,
    labelsize: u32,
}

impl ChartDisplay {
    fn new (timescale: u32, timeinterval: u32, labelsize: u32) -> Self
    {
        let lab = canvas::Text::default();
        let lh = lab.size.0 as f64 * 1.5;
        Self {lheight: lh, topofcabs: 0.0, cabheight: 0.0,
                             bottomofcabs: 0.0, numberofcabs: 0, 
                             cabarray: HashMap::new(), topofchart: 0.0, 
                             chartheight: 0.0, bottomofchart: 0.0, 
                             totallength: 0.0, chartstationoffset: 0.0,
                             topofstorage: 0.0, storagetrackheight: 0.0, 
                             bottomofstorage: 0.0, numberofstoragetracks: 0,
                             storageoffset: 0.0, stationarray: HashMap::new(),
                             storagearray: HashMap::new(),
                             timescale: if timescale == 0 {1440} else {timescale}, 
                             timeinterval: if timeinterval == 0 {15} else {timeinterval},
                             labelsize: labelsize, }
    }
    fn _buildTimeLine(&mut self,frame: &mut canvas::Frame)
    {
        let numIncrs =  (((self.timescale as f64) + (self.timeinterval as f64)) / 
                            (self.timeinterval as f64)) as i32;
        let cwidth = (numIncrs * 20) + self.labelsize as i32 + 20;
        let scrollWidth = cwidth;
        let topOff = 0;
        for m in (0..=self.timescale).step_by(60) {
            let mx = (self.labelsize as f64) + ( ( ((m as f64) / (self.timeinterval as f64)) * 20.0)) + 4.0;
            let hourlab = format!("{:2}", m / 60);
            // draw @ mx,0, anchored at top
            let mut label = Text::from(hourlab);
            label.position = Point::new(mx as f32,0.0);
            label.draw_with(|path,color|frame.fill(&path,color));
            //self.hull.create_text( mx, 0.0, -anchor("n") -text(format!("{:2}", m / 60)) -tags("TimeLine"))?;
         }
    }
    pub fn deleteWholeChart(&mut self) {
        self.topofcabs = 0.0;
        self.cabheight = 0.0;
        self.bottomofcabs = 0.0;
        self.numberofcabs = 0;
        self.cabarray.clear();
        self.topofchart = 0.0;
        self.chartheight = 0.0;
        self.bottomofchart = 0.0;
        self.totallength = 0.0;
        self.chartstationoffset = 0.0;
        self.topofstorage = 0.0;
        self.storagetrackheight = 0.0;
        self.bottomofstorage = 0.0;
        self.numberofstoragetracks = 0;
        self.storageoffset = 0.0;
        self.totallength = 0.0;
        self.stationarray.clear();
        self.storagearray.clear();
        self.totallength = 0.0;
    }
    pub fn ComputeTotalSize(timetable: &TimeTableSystem) -> (f32, f32) 
    {
        let timescale = timetable.TimeScale();
        let timeinterval = timetable.TimeInterval();
        let totallength = timetable.TotalLength();
        let chartheight = (totallength * 20.0) + 20.0;
        let numIncrs =  (((timescale as f64) + (timeinterval as f64)) / 
                            (timeinterval as f64)) as i32;
        let cwidth = (numIncrs * 20) + 100 + 20;
        let framewidth = cwidth as f32;
        let lab = canvas::Text::default();
        let lheight = lab.size.0 as f64 * 1.5;
        let topOff = lheight;
        let topofcabs = (topOff + 10.0) as f64;
        let cabheight = (lheight + 20.0)
                            +timetable.NumberOfCabs() as f64 * lheight;
        let topOff = chartheight + cabheight;
        let topofstorage = topOff as f64 + 10.0;
        let storageoffset = topofstorage;
        let topOff = topofstorage;
        let mut numstorage = 0;
        for station in timetable.StationsIter() {
            for storage in station.storagetracks() {
                numstorage += 1;
            }
        }
        let storagetrackheight = (lheight + 20.0) + (numstorage as f64 * lheight);
        let bottomofstorage = topofstorage + storagetrackheight;
        let cheight = bottomofstorage;
        let frameheight = cheight as f32;
        eprintln!("*** ChartDisplay::ComputeTotalSize returns: ({},{})",framewidth,frameheight);
        (framewidth, frameheight)
    }
    
    fn buildWholeChart(&mut self, renderer: &Renderer, 
                        timetable: &TimeTableSystem) -> Vec<canvas::Geometry> {
        self.deleteWholeChart();
        self.timescale = timetable.TimeScale();
        self.timeinterval = timetable.TimeInterval();
        self.totallength = timetable.TotalLength();
        self.chartheight = (self.totallength * 20.0) + 20.0;
        let numIncrs =  (((self.timescale as f64) + (self.timeinterval as f64)) / 
                            (self.timeinterval as f64)) as i32;
        let cwidth = (numIncrs * 20) + self.labelsize as i32 + 20;
        let framewidth = cwidth as f32;
        let topOff = self.lheight;
        self.topofcabs = (topOff + 10.0) as f64;
        self.cabheight = (self.lheight + 20.0)
                            +timetable.NumberOfCabs() as f64 * self.lheight;
        let topOff = self.chartheight + self.cabheight;
        self.topofstorage = topOff as f64 + 10.0;
        self.storageoffset = self.topofstorage;
        let topOff = self.topofstorage;
        let mut numstorage = 0;
        for station in timetable.StationsIter() {
            for storage in station.storagetracks() {
                numstorage += 1;
            }
        }
        self.storagetrackheight = (self.lheight + 20.0) + 
                                    (numstorage as f64 * self.lheight);
        self.bottomofstorage = self.topofstorage + self.storagetrackheight;
        let cheight = self.bottomofstorage;
        let frameheight = cheight as f32;

        eprintln!("*** ChartDisplay::buildWholeChart frame size is ({},{})",framewidth,frameheight);
        let mut frame = canvas::Frame::new(renderer, Size::new(framewidth, frameheight));

        self._buildTimeLine(&mut frame);
        //self._buildCabs(&mut frame);
        //self._buildChart(&mut frame);
        //self._buildStorageTracks(&mut frame);

        for (name, cab) in timetable.CabsIter() {
            //self.addACab(&mut frame,cab);
        }
        let mut sindex = 0;
        for station in timetable.StationsIter() {
            //self.addAStation(&mut frame,station,sindex);
            sindex += 1;
        }
        for (number,train) in timetable.TrainsIter() {
            //self.addATrain(&mut frame,timetable,train);
        }
        vec![frame.into_geometry()]
    }
}


impl<Message> canvas::Program<Message> for TimeTableGUI {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor
    ) -> Vec<canvas::Geometry> {
        //eprintln!("*** TimeTableGUI::draw: bounds is {:?}",bounds);
        let mut cd = ChartDisplay::new(self.timetable.TimeScale(),
                                   self.timetable.TimeInterval(),
                                   100);
        cd.buildWholeChart(renderer,&self.timetable)
    }
}

struct TimeTableGUI {
    panes: pane_grid::State<Pane>,
    timetable: TimeTableSystem,
}

impl TimeTableGUI {
    fn new() -> Self {
        let (mut panes, main) = pane_grid::State::new(Pane::Main);
        let Some((buttonpane,split)) = panes.split(Axis::Vertical,main,Pane::Buttons) else {panic!("failed to create buttons");};
        panes.resize(split,0.7490561);
        TimeTableGUI {
            panes, 
            timetable: TimeTableSystem::new(String::from("Dummy"),1440,15),
        }
    }
    fn update(&mut self, message: Message) {
        eprintln!("*** TimeTableGUI::update(): message is {:?}",message);
        match message {
            Message::FileSelected(File::Open) => {
                match TimeTableSystem::old("LJandBS.tt") {
                    Ok(tt) => {self.timetable = tt;
                                eprintln!("{} loaded: {}","LJandBS.tt",
                                            self.timetable.Name());
                    },
                    Err(err) => {eprintln!("Count not open LJandBS.tt: {}",
                                                err.to_string());
                    },
                }
            },
            Message::FileSelected(File::Exit) => exit(0),
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                //eprintln!("*** TimeTableGUI::update(): split is {:?}",split);
                self.panes.resize(split, ratio);
            }
            _ => (),
        }
    }

    fn white(_:&Theme, _:button::Status) -> Style {
        Style {background:Some(Background::Color(Color::from_rgb8(255,255,255))),
                                     ..Style::default()}
    } 
    fn menubar() -> Element<'static, Message> {
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
        ].into()
    }
    fn toolbar() -> Element<'static, Message> {
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
        ].into()        
    }
    fn buttonbox() -> Element<'static, Message> {
        column![
            button("Add a new train")
                .style(Self::white)
                .on_press(Message::TrainsSelected(Trains::AddTrain)),
            button("Delete an existing train")
                .style(Self::white)
                .on_press(Message::TrainsSelected(Trains::DeleteTrain)),
            button("Set Duplicate Station")
                .style(Self::white)
                .on_press(Message::StationsSelected(Stations::SetDuplicateStation)),
            button("Clear Duplicate Station")
                .style(Self::white)
                .on_press(Message::StationsSelected(Stations::ClearDuplicateStation)),
            button("Add Storage Track")
                .style(Self::white)
                .on_press(Message::StationsSelected(Stations::AddStorageTrack)),
            button("Add A Cab")
                .style(Self::white)
                .on_press(Message::CabsSelected(Cabs::AddACab)),
            button("Create New Note")
                .style(Self::white)
                .on_press(Message::NotesSelected(Notes::CreateNewNote)),
            button("Edit Existing Note")
                .style(Self::white)
                .on_press(Message::NotesSelected(Notes::EditExistingNote)),
            button("Add note to train")
                .style(Self::white)
                .on_press(Message::NotesSelected(Notes::AddNoteToTrain)),
            button("Add note to train at station stop")
                .style(Self::white)
                .on_press(Message::NotesSelected(Notes::AddNoteToTrainAtStation)),
            button("Remove note from train")
                .style(Self::white)
                .on_press(Message::NotesSelected(Notes::RemoveNoteFromTrain)),
            button("Remove note from train at station stop")
                .style(Self::white)
                .on_press(Message::NotesSelected(Notes::RemoveNoteFromTrainAtStation)),
            button("Quit -- Exit NOW")
                .style(Self::white)
                .on_press(Message::FileSelected(File::Exit)),
        ].into()
    }
    fn view(&self) -> Element<'_, Message> {

        let pane_grid = pane_grid(&self.panes, |pane, state, is_maximized| {
                match state {
                    Pane::Main => {
                        let (framewidth, frameheight) = 
                            ChartDisplay::ComputeTotalSize(&self.timetable);
                        eprintln!("*** TimeTableGUI::view framewidth = {}, frameheight is {}",framewidth,frameheight);
                        pane_grid::Content::new(
                            scrollable(
                                canvas(self)
                                    .width(framewidth)
                                    .height(frameheight)
                            )
                        )},
                    Pane::Buttons => 
                        pane_grid::Content::new(Self::buttonbox()),
                }
            })
            .spacing(10)
            /*.on_click(Message::Clicked)*/
            /*.on_drag(Message::Dragged)*/
            .on_resize(10, Message::Resized);
        
        column![
            Self::menubar(),
            Self::toolbar(),
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
