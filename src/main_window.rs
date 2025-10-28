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
//  Last Modified : <251027.1748>
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
//    MERCHANTABILITY or FITNESS FOR A PARTCULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    You should have received a copy of the GNU General Public License
//    along with this program; if not, write to the Free Software
//    Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
// 
//
//////////////////////////////////////////////////////////////////////////////


mod menubar;
use crate::main_window::menubar::*;
use time_table::*;
use time_table::cab::*;
use time_table::station::*;
use time_table::train::*;
use iced::widget::{pane_grid, pick_list, button, column, row, text, Column,
                   scrollable, responsive, container, image, canvas}; 
use iced::widget::pane_grid::{Axis,PaneGrid};
use iced::{Background,Theme,Center, Color, Element, Fill, Size, Subscription,
            Renderer, Rectangle, mouse, Point};
use iced::widget::button::Style;
use iced::widget::canvas::{Text,Path,Stroke,Event};
use iced::widget::canvas::path::lyon_path;
use iced::alignment::Vertical;
use iced::keyboard;
use std::process::exit;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::sync::{LazyLock, Mutex};


#[derive(Debug, Clone, Copy)]
pub enum Message {
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

#[derive(Debug, Clone)]
pub enum Action {
    //            defext initdir initfile title
    OpenFileSelect(String,String,String,String)
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

static COLOR_MAP: Mutex<HashMap<&'static str, (u8,u8,u8), BuildHasherDefault<DefaultHasher>>> =
    Mutex::new(HashMap::with_hasher(BuildHasherDefault::new()));
fn InitColorMap() {
    if COLOR_MAP.lock().unwrap().get("black").is_some() {return;}
    COLOR_MAP.lock().unwrap().insert("black",(0,0,0));
    COLOR_MAP.lock().unwrap().insert("white",(255,255,255));
    COLOR_MAP.lock().unwrap().insert("grey",( 190, 190, 190));
    COLOR_MAP.lock().unwrap().insert("blue",(   0,   0, 255));
    COLOR_MAP.lock().unwrap().insert("cyan",(   0, 255, 255));
    COLOR_MAP.lock().unwrap().insert("green",(  0, 255,   0));
    COLOR_MAP.lock().unwrap().insert("yellow",( 255, 255,  0));
    COLOR_MAP.lock().unwrap().insert("violet",( 238, 130, 238));
    COLOR_MAP.lock().unwrap().insert("magenta",( 255,   0, 255));
    COLOR_MAP.lock().unwrap().insert("purple",( 160,  32, 240));
    COLOR_MAP.lock().unwrap().insert("red",(  255,   0,   0));
}

fn LookupColorByName(name: &str) -> Color {
    let (r,g,b) = match COLOR_MAP.lock().unwrap().get(name) {
        None => (0,0,0),
        Some(rgb) => *rgb,
    };
    Color::from_rgba8(r,g,b,1.0)
}



impl ChartDisplay {
    fn new (timescale: u32, timeinterval: u32, labelsize: u32) -> Self
    {
        let lab = canvas::Text::default();
        let lh = lab.size.0 as f64 * 1.5;
        //eprintln!("*** ChartDisplay::new(): lh is {}",lh);
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
        let chartheight = (totallength * 20.0) + 40.0;
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
        let storagetrackheight = (lheight) + (numstorage as f64 * lheight);
        let bottomofstorage = topofstorage + storagetrackheight;
        let cheight = bottomofstorage + lheight;
        let frameheight = cheight as f32;
        eprintln!("*** ChartDisplay::ComputeTotalSize returns: ({},{})",framewidth,frameheight);
        (framewidth, frameheight)
    }
    pub fn addACab(&mut self,frame: &mut canvas::Frame,cab: &Cab,
                    cabindex: usize) 
    {
        let cabyoff = (self.lheight*0.5) + (cabindex as f64 * self.lheight);
        self.numberofcabs += 1;
        let cabName = cab.Name();
        let cabColor = cab.Color();
        let cy = cabyoff+self.topofcabs;
        self.cabarray.insert(CabArrayIndex::Y(cabName.clone()),cy);
        let r = self.labelsize as f64 + ((self.timescale as f64 / self.timeinterval as f64) * 20.0);
        let line = canvas::Path::line(
                Point::new(self.labelsize as f32,cy as f32),
                Point::new(r as f32,cy as f32));
        let stroke = Stroke::default()
                    .with_color(LookupColorByName(cabColor.as_str()))
                    .with_width(4.0);
        frame.stroke(&line,stroke); 
        let mut label = Text::from(cabName);
        label.position = Point::new(0.0,cy as f32);
        label.vertical_alignment = Vertical::Center;
        label.color = LookupColorByName(cabColor.as_str());
        label.draw_with(|path,color|frame.fill(&path,color));
    }
    fn _labelWidthCheck(name: &str,width: f32) -> bool
    {
        let mut result: bool = true; 
        let text = Text::from(name);
        text.draw_with(|path,_| {
            for e in path.raw().iter() {
                match e {
                    lyon_path::Event::Begin { at } => {
                        if at.x > width {result = false; break;}
                    },
                    lyon_path::Event::Line { from, to } => {
                        if from.x > width ||
                           to.x > width {result = false; break;} 
                    },
                    lyon_path::Event::Quadratic { from, ctrl, to } => {
                        if from.x > width ||
                           to.x > width {result = false; break;} 
                    },
                    lyon_path::Event::Cubic {from, ctrl1, ctrl2, to, } => {
                        if from.x > width ||
                           to.x > width {result = false; break;} 
                    },
                    lyon_path::Event::End { last, first, close } => {
                        if last.x > width ||
                           first.x > width {result = false; break;} 
                    },   
                }
            }
        });
        result
    }
    fn _MakeLabelBox(label: &Text) -> Path
    {
        let mut topLeft = Point::new(99999999.9,99999999.9);
        let mut size    = Size::new(0.0,0.0);
        label.draw_with(|path,_| {
            for e in path.raw().iter() {
                match e {
                    lyon_path::Event::Begin { at } => {
                        if at.x < topLeft.x {topLeft.x = at.x;}
                        if at.y < topLeft.y {topLeft.y = at.y;}
                    },
                    lyon_path::Event::Line { from, to } => {
                        if from.x < topLeft.x {topLeft.x = from.x;}
                        if from.y < topLeft.y {topLeft.y = from.y;}
                        if to.x < topLeft.x {topLeft.x = to.x;}
                        if to.y < topLeft.y {topLeft.y = to.y;}
                    },
                    lyon_path::Event::Quadratic { from, ctrl, to } => {
                        if from.x < topLeft.x {topLeft.x = from.x;}
                        if from.y < topLeft.y {topLeft.y = from.y;}
                        if to.x < topLeft.x {topLeft.x = to.x;}
                        if to.y < topLeft.y {topLeft.y = to.y;}
                    },
                    lyon_path::Event::Cubic {from, ctrl1, ctrl2, to, } => {
                        if from.x < topLeft.x {topLeft.x = from.x;}
                        if from.y < topLeft.y {topLeft.y = from.y;}
                        if to.x < topLeft.x {topLeft.x = to.x;}
                        if to.y < topLeft.y {topLeft.y = to.y;}
                    },
                    lyon_path::Event::End { last, first, close } => {
                        if last.x < topLeft.x {topLeft.x = last.x;}
                        if last.y < topLeft.y {topLeft.y = last.y;}
                        if first.x < topLeft.x {topLeft.x = first.x;}
                        if first.y < topLeft.y {topLeft.y = first.y;}
                    },
                }
            }
            for e in path.raw().iter() {
                match e {
                    lyon_path::Event::Begin { at } => {
                        let sx = at.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = at.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                    },
                    lyon_path::Event::Line { from, to } => {
                        let sx = from.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = from.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                        let sx = to.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = to.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                    },
                    lyon_path::Event::Quadratic { from, ctrl, to } => {
                        let sx = from.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = from.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                        let sx = to.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = to.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                    },
                    lyon_path::Event::Cubic {from, ctrl1, ctrl2, to, } => {
                        let sx = from.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = from.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                        let sx = to.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = to.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                    },
                    lyon_path::Event::End { last, first, close } => {
                        let sx = last.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = last.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                        let sx = first.x - topLeft.x;
                        if sx > size.width {size.width = sx;}
                        let sy = first.y - topLeft.y;
                        if sy > size.height {size.height = sy;}
                    },
                }
            }
        });
        Path::rectangle(topLeft,size)
    }
    pub fn addAStation(&mut self,frame: &mut canvas::Frame,station: &Station,sindex: usize)
    {
        let mut name: &str = &station.Name();
        let smile = station.SMile(); 
        if smile > self.totallength {self.totallength = smile;} 
        let offset = self.topofchart + 20.0;
        let y = offset+(smile * 20.0);
        self.stationarray.insert(StationArrayIndex::Y(sindex),y);
        self.stationarray.insert(StationArrayIndex::Smile(sindex),smile);
        let lsize = self.labelsize as f32;
        loop {
            if Self::_labelWidthCheck(name,lsize) {break;}
            name = &name[0..name.len()-1];
        }
        let mut labelText = Text::from(name);
        labelText.position = Point::new(0.0,y as f32);
        labelText.vertical_alignment = Vertical::Center;
        let labelBox = Self::_MakeLabelBox(&labelText);
        labelText.draw_with(|path,color|frame.fill(&path,color));
        let stroke = Stroke::default().with_color(Color::BLACK).with_width(1.0);
        frame.stroke(&labelBox,stroke);
        let r = self.labelsize as f64 +
                    (((self.timescale as f64 / self.timeinterval as f64) * 20.0));
        let line = canvas::Path::line(
                Point::new(self.labelsize as f32,y as f32),
                Point::new(r as f32,y as f32));
        let stroke = Stroke::default() 
                .with_color(Color::BLACK)
                .with_width(2.0);
        frame.stroke(&line,stroke);
        for storage in station.storagetracks() {
            self.addAStorageTrack(frame,station,storage);
        }
    }
    pub fn addAStorageTrack(&mut self,frame: &mut canvas::Frame,
                    station: &Station, track: &StorageTrack)
    {
        let storageyoff = (self.lheight*0.5) + (self.numberofstoragetracks as f64 * self.lheight);
        self.numberofstoragetracks += 1;
        let stationName = station.Name();
        let trackName   = track.Name();
        let nameOnChart = Self::_formNameOnChart(self.labelsize as f32,
                                                 stationName.as_str(), 
                                                 trackName.as_str());
        let y = storageyoff + self.topofstorage;
        self.storagearray.insert(StorageArrayIndex::Y(stationName.clone(),
                                                      trackName.clone()),y);
        let mut label = Text::from(nameOnChart);
        label.position = Point::new(0.0,y as f32);
        label.vertical_alignment = Vertical::Center;
        label.draw_with(|path,color|frame.fill(&path,color));
        let stroke = Stroke::default().with_color(Color::BLACK).with_width(4.0);
        let r = self.labelsize as f64 + (((self.timescale as f64 / self.timeinterval as f64) * 20.0));
        let line = canvas::Path::line(
            Point::new(self.labelsize as f32,y as f32),
            Point::new(r as f32,y as f32));
        frame.stroke(&line,stroke);
    }
    fn _formNameOnChart(lsize: f32,sn_: &str,tn_: &str) -> String
    {
        let mut sn = &sn_[0..];
        let mut tn = &tn_[0..];
        loop {
            let l1 = format!("{}:{}",sn,tn);
            let l2 = format!("{}:",sn);
            if Self::_labelWidthCheck(l1.as_str(),lsize) ||
               Self::_labelWidthCheck(l2.as_str(),lsize / 2.0) {break;}
            sn = &sn[0..sn.len()-1]; 
        }
        loop {
            let l1 = format!("{}:{}",sn,tn);
            if Self::_labelWidthCheck(l1.as_str(),lsize as f32) {
                break;
            }
            tn = &tn[0..tn.len()-1];
        }
        format!("{}:{}",sn,tn)
    }

    pub fn addATrain(&mut self,frame: &mut canvas::Frame,timetable: &TimeTableSystem,train:&Train)
    {
        let lastX = self.labelsize as f64 + ((self.timescale as f64 / self.timeinterval as f64) * 20.0) + 4.0;
        let firstX = self.labelsize as f64 + 4.0;

        let mut timeX = -1.0;
        let mut stationY = -1.0;
        let mut rStationY = -1.0;
        // The next two variables are not referenced the first time
        // through the loop (because timeX is initialized to -1),
        // but the dumb compiler is afraid they might be referenced
        // when uninitialized.
        let mut color: String = String::from("black");
        let mut cabName: String = String::new();
        let departure = train.Departure();
        let mut oldDepart = -1.0;
        let mut oldSmile = -1.0;
        let speed = train.Speed();
        for stop in train.StopIter() {
            let sindex = stop.StationIndex();
            let station = timetable.IthStation(sindex).unwrap();
            let smile = station.SMile();
            let rSindex = station.DuplicateStationIndex();
            let (rStation,rsmile,newRStationY) =
                match rSindex {
                    None => (None,-1.0,-1.0),
                    Some(rsind) => {
                        let rStation = timetable.IthStation(rsind).unwrap();
                        let rsmile = rStation.SMile();
                        let newRStationY = self.stationarray.get(&StationArrayIndex::Y(rsind)).unwrap();
                        (Some(rStation),rsmile,*newRStationY)
                    },
            };
            let departcab = stop.TheCab();
            let newcolor: String;
            let newname: String;
            //let newColor: &str;
            //let newCabName: &str;
            match departcab {
                None => {
                    newcolor = String::from("black");
                    newname = String::new();
                },                    
                Some(cab) => {
                    newcolor = cab.Color();
                    newname =  cab.Name();
                },
            };
            let newStationY = self.stationarray.get(&StationArrayIndex::Y(sindex)).unwrap();
            let arrival: f64 = if oldDepart >= 0.0 {
                oldDepart + (smile - oldSmile).abs() * (speed as f64 / 60.0)
            } else {
                departure as f64
            };
            let storage: Option<&StorageTrack>;
            let rstorage: Option<&StorageTrack>;
            match stop.Flag() {
                StopFlagType::Origin => {
                    let depart = departure as f64;
                    storage = station.FindTrackTrainIsStoredOn(train.Number(),depart,depart);
                    if rStation.is_some() {
                        rstorage = rStation.unwrap().FindTrackTrainIsStoredOn(train.Number(),depart,depart);
                    } else {
                        rstorage = None;
                    }
                },
                StopFlagType::Terminate => {
                    storage = station.FindTrackTrainIsStoredOn(train.Number(),arrival,arrival);
                    if rStation.is_some() {
                        rstorage = rStation.unwrap().FindTrackTrainIsStoredOn(train.Number(),arrival,arrival);
                    } else {
                        rstorage = None;
                    }
                },
                StopFlagType::Transit => {
                    storage = None;
                    rstorage = None;
                },
            };
            if storage.is_some() {
                let stationName = station.Name();
                let trackName   = storage.unwrap().Name();
                let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                let occupiedA = storage.unwrap().IncludesTime(arrival);
                let occupiedD = storage.unwrap().IncludesTime(departure as f64);
                if occupiedA.is_some() &&
                   occupiedA.unwrap().TrainNum() == train.Number() {
                    let from = occupiedA.unwrap().From();
                    let to   = occupiedA.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(newcolor.as_str()))
                        .with_width(8.0);
                    if toX > fromX {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    } else {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(lastX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                        let line = canvas::Path::line(
                            Point::new(firstX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    }
                }
                if occupiedD.is_some() &&
                   occupiedD.unwrap().TrainNum2() == train.Number() {
                    let from = occupiedD.unwrap().From();
                    let to   = occupiedD.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(newcolor.as_str()))
                        .with_width(8.0);
                    if toX > fromX {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    } else {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(lastX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                        let line = canvas::Path::line(
                            Point::new(firstX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    }
                }
            }
            if rstorage.is_some() {
                let stationName = rStation.unwrap().Name();
                let trackName   = rstorage.unwrap().Name();
                let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                let occupiedA = rstorage.unwrap().IncludesTime(arrival);
                let occupiedD = rstorage.unwrap().IncludesTime(departure as f64);
                if occupiedA.is_some() &&
                   occupiedA.unwrap().TrainNum() == train.Number() {
                    let from = occupiedA.unwrap().From();
                    let to   = occupiedA.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(newcolor.as_str()))
                        .with_width(8.0);
                    if toX > fromX {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    } else {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(lastX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                        let line = canvas::Path::line(
                            Point::new(firstX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    }
                }
                if occupiedD.is_some() &&
                   occupiedD.unwrap().TrainNum2() == train.Number() {
                    let from = occupiedD.unwrap().From();
                    let to   = occupiedD.unwrap().Until();
                    let fromX = self.labelsize as f64 +
                                    ((from / self.timeinterval as f64) * 20.0) + 4.0;
                    let toX   = self.labelsize as f64 +
                                    ((to / self.timeinterval as f64) * 20.0) + 4.0;
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(newcolor.as_str()))
                        .with_width(8.0);
                    if toX > fromX {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    } else {
                        let line = canvas::Path::line(
                            Point::new(fromX as f32,sy as f32),
                            Point::new(lastX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                        let line = canvas::Path::line(
                            Point::new(firstX as f32,sy as f32),
                            Point::new(toX as f32,sy as f32));
                        frame.stroke(&line,stroke);
                    }
                }
            }
            let mut newTimeX = self.labelsize as f64 + ((arrival / self.timeinterval as f64) * 20.0) + 4.0;
            if timeX >= 0.0 {
                if newTimeX > timeX {
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(color.as_str()))
                        .with_width(4.0);
                    let line = canvas::Path::line(
                            Point::new(timeX as f32,stationY as f32),
                            Point::new(newTimeX as f32,*newStationY as f32));
                    frame.stroke(&line,stroke);
                    if rStationY >= 0.0 && newRStationY >= 0.0 {
                        let line = canvas::Path::line(
                            Point::new(timeX as f32,rStationY as f32),
                            Point::new(newTimeX as f32,newRStationY as f32));
                        frame.stroke(&line,stroke);
                    }
                    match self.cabarray.get(&CabArrayIndex::Y(cabName.clone())) {
                        None => (),
                        Some(cy) => {
                            let stroke = Stroke::default() 
                                .with_color(LookupColorByName(color.as_str()))
                                .with_width(8.0);
                            let line = canvas::Path::line(
                                Point::new(timeX as f32,*cy as f32),
                                Point::new(newTimeX as f32,*cy as f32));
                            frame.stroke(&line,stroke);
                        },
                    }
                } else {
                    let unwrapNX = newTimeX - lastX;
                    let slope = (newStationY - stationY) as f64 / (unwrapNX - timeX) as f64;
                    let midY = stationY + (slope * (lastX - timeX));
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(color.as_str()))
                        .with_width(4.0);
                    let line = canvas::Path::line(
                        Point::new(timeX as f32,stationY as f32),
                        Point::new(lastX as f32,midY as f32));
                    frame.stroke(&line,stroke);
                    let line = canvas::Path::line(
                        Point::new(firstX as f32,midY as f32),
                        Point::new(newTimeX as f32,*newStationY as f32));
                    frame.stroke(&line,stroke);
                    if rStationY >= 0.0 && newRStationY >= 0.0 {
                        let slope = (newRStationY - rStationY) as f64 / 
                                    (unwrapNX - timeX) as f64;
                        let midY = rStationY + (slope * (lastX - timeX));
                        let line = canvas::Path::line(
                                Point::new(timeX as f32,rStationY as f32),
                                Point::new(lastX as f32,midY as f32));
                        frame.stroke(&line,stroke); 
                        let line = canvas::Path::line(
                                Point::new(firstX as f32,midY as f32),
                                Point::new(newTimeX as f32,newRStationY as f32));
                        frame.stroke(&line,stroke); 
                    }
                    match self.cabarray.get(&CabArrayIndex::Y(cabName.clone())) {
                        None => (),
                        Some(cy) => {
                            let stroke = Stroke::default()
                                .with_color(LookupColorByName(color.as_str()))
                                .with_width(8.0);
                            let line = canvas::Path::line(
                                Point::new(timeX as f32,*cy as f32),
                                Point::new(newTimeX as f32,*cy as f32));
                            frame.stroke(&line,stroke);
                        },
                    }
                }
            }
            timeX = newTimeX;
            cabName = newname;
            color = newcolor;
            stationY = *newStationY;
            rStationY = newRStationY;
            let depart = stop.Departure(arrival);
            if depart > arrival {
                let (cy, dontdrawcab): (f64, bool) = 
                    match self.cabarray.get(&CabArrayIndex::Y(cabName.clone())) {
                        None => (0.0, true),
                        Some(cy) => (*cy, false),
                };
                newTimeX = self.labelsize as f64 + ((depart / self.timeinterval as f64) * 20.0) + 4.0;
                if newTimeX > timeX {
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(color.as_str()))
                        .with_width(4.0);
                    let line = canvas::Path::line(
                        Point::new(timeX as f32,stationY as f32),
                        Point::new(newTimeX as f32,stationY as f32));
                    frame.stroke(&line,stroke);
                    if rStationY >= 0.0 {
                        let line = canvas::Path::line(
                            Point::new(timeX as f32,rStationY as f32),
                            Point::new(newTimeX as f32,rStationY as f32));
                        frame.stroke(&line,stroke);
                    }
                    if !dontdrawcab {
                        let stroke = Stroke::default()
                            .with_color(LookupColorByName(color.as_str()))
                            .with_width(8.0);
                        let line = canvas::Path::line(
                            Point::new(timeX as f32,cy as f32),
                            Point::new(newTimeX as f32,cy as f32));
                        frame.stroke(&line,stroke);
                    }
                } else {
                    let stroke = Stroke::default()
                        .with_color(LookupColorByName(color.as_str()))
                        .with_width(4.0);
                    let line = canvas::Path::line(
                        Point::new(timeX as f32,stationY as f32),
                        Point::new(lastX as f32,stationY as f32));
                    frame.stroke(&line,stroke);
                    let line = canvas::Path::line(
                        Point::new(firstX as f32,stationY as f32),
                        Point::new(newTimeX as f32,stationY as f32));
                    frame.stroke(&line,stroke);                    
                    if rStationY >= 0.0 {
                        let line = canvas::Path::line(
                            Point::new(timeX as f32,rStationY as f32),
                            Point::new(lastX as f32,rStationY as f32));
                        frame.stroke(&line,stroke);
                        let line = canvas::Path::line(
                            Point::new(firstX as f32,rStationY as f32),
                            Point::new(newTimeX as f32,rStationY as f32));
                        frame.stroke(&line,stroke);
                    }
                    if !dontdrawcab {
                        let stroke = Stroke::default()
                            .with_color(LookupColorByName(color.as_str()))
                            .with_width(8.0);
                        let line = canvas::Path::line(
                            Point::new(timeX as f32,cy as f32),
                            Point::new(lastX as f32,cy as f32));
                        frame.stroke(&line,stroke);
                        let line = canvas::Path::line(
                            Point::new(firstX as f32,cy as f32),
                            Point::new(newTimeX as f32,cy as f32));
                        frame.stroke(&line,stroke);
                    }
                }
                let storage = station.FindTrackTrainIsStoredOn(train.Number(),arrival,depart);
                if storage.is_some() {
                    let stationName = station.Name();
                    let trackName   = storage.unwrap().Name();
                    let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                    let occupiedA = storage.unwrap().IncludesTime(arrival);
                    let occupiedD = storage.unwrap().IncludesTime(depart);
                    if occupiedA.is_some() &&
                       occupiedA.unwrap().TrainNum() == train.Number() {
                        let from = occupiedA.unwrap().From();
                        let to   = occupiedA.unwrap().Until();
                        let fromX = self.labelsize as f64 +
                                        ((from / self.timeinterval as f64) * 20.0) + 4.0;
                        let toX   = self.labelsize as f64 +
                                        ((to / self.timeinterval as f64) * 20.0) + 4.0;
                        let stroke = Stroke::default()
                            .with_color(LookupColorByName(color.as_str()))
                            .with_width(8.0);
                        
                        if toX > fromX {
                            let line = canvas::Path::line(
                                Point::new(fromX as f32,sy as f32),
                                Point::new(toX as f32,sy as f32));
                            frame.stroke(&line,stroke);
                        } else {
                            let line = canvas::Path::line(
                                Point::new(fromX as f32,sy as f32),
                                Point::new(lastX as f32,sy as f32));
                            frame.stroke(&line,stroke);
                            let line = canvas::Path::line(
                                Point::new(firstX as f32,sy as f32),
                                Point::new(toX as f32,sy as f32));
                            frame.stroke(&line,stroke);
                        }
                    }
                    if occupiedD.is_some() &&
                       occupiedA != occupiedD &&
                       occupiedD.unwrap().TrainNum() == train.Number() {
                        let from = occupiedD.unwrap().From();
                        let to   = occupiedD.unwrap().Until();
                        let fromX = self.labelsize as f64 +
                                        ((from / self.timeinterval as f64) * 20.0) + 4.0;
                        let toX   = self.labelsize as f64 +
                                        ((to / self.timeinterval as f64) * 20.0) + 4.0;
                        let stroke = Stroke::default()
                            .with_color(LookupColorByName(color.as_str()))
                            .with_width(8.0);
                        if toX > fromX {
                            let line = canvas::Path::line(
                                Point::new(fromX as f32,sy as f32),
                                Point::new(toX as f32,sy as f32));
                            frame.stroke(&line,stroke);
                        } else {
                            let line = canvas::Path::line(
                                Point::new(fromX as f32,sy as f32),
                                Point::new(lastX as f32,sy as f32));
                            frame.stroke(&line,stroke);
                            let line = canvas::Path::line(
                                Point::new(firstX as f32,sy as f32),
                                Point::new(toX as f32,sy as f32));
                            frame.stroke(&line,stroke);
                        }
                    }
                        
                }
                if rStation.is_some() {
                    let storage = rStation
                                    .unwrap()
                                    .FindTrackTrainIsStoredOn(train.Number(),
                                                              arrival,
                                                              depart);
                    if storage.is_some() {
                        let stationName = rStation.unwrap().Name();
                        let trackName   = storage.unwrap().Name();
                        let sy = *self.storagearray.get(&StorageArrayIndex::Y(stationName,trackName)).unwrap();
                        let occupiedA = storage.unwrap().IncludesTime(arrival);
                        let occupiedD = storage.unwrap().IncludesTime(depart);
                        if occupiedA.is_some() &&
                           occupiedA.unwrap().TrainNum() == train.Number() {
                            let from = occupiedA.unwrap().From();
                            let to   = occupiedA.unwrap().Until();
                            let fromX = self.labelsize as f64 +
                                            ((from / self.timeinterval as f64) * 20.0) + 4.0;
                            let toX   = self.labelsize as f64 +
                                            ((to / self.timeinterval as f64) * 20.0) + 4.0;
                            let stroke = Stroke::default()
                                .with_color(LookupColorByName(color.as_str()))
                                .with_width(8.0);
                            if toX > fromX {
                                let line = canvas::Path::line(
                                    Point::new(fromX as f32,sy as f32),
                                    Point::new(toX as f32,sy as f32));
                                frame.stroke(&line,stroke);
                            } else {
                                let line = canvas::Path::line(
                                    Point::new(fromX as f32,sy as f32),
                                    Point::new(lastX as f32,sy as f32));
                                frame.stroke(&line,stroke);
                                let line = canvas::Path::line(
                                    Point::new(firstX as f32,sy as f32),
                                    Point::new(toX as f32,sy as f32));
                                frame.stroke(&line,stroke);
                            }
                        }
                        if occupiedD.is_some() &&
                           occupiedA != occupiedD &&
                           occupiedD.unwrap().TrainNum() == train.Number() {
                            let from = occupiedD.unwrap().From();
                            let to   = occupiedD.unwrap().Until();
                            let fromX = self.labelsize as f64 +
                                            ((from / self.timeinterval as f64) * 20.0) + 4.0;
                            let toX   = self.labelsize as f64 +
                                            ((to / self.timeinterval as f64) * 20.0) + 4.0;
                            let stroke = Stroke::default()
                                .with_color(LookupColorByName(color.as_str()))
                                .with_width(8.0);
                            if toX > fromX {
                                let line = canvas::Path::line(
                                    Point::new(fromX as f32,sy as f32),
                                    Point::new(toX as f32,sy as f32));
                                frame.stroke(&line,stroke);
                            } else {
                                let line = canvas::Path::line(
                                    Point::new(fromX as f32,sy as f32),
                                    Point::new(lastX as f32,sy as f32));
                                frame.stroke(&line,stroke);
                                let line = canvas::Path::line(
                                    Point::new(firstX as f32,sy as f32),
                                    Point::new(toX as f32,sy as f32));
                                frame.stroke(&line,stroke);
                            }
                        }
                    }
                }
                timeX = newTimeX;
            }
            oldDepart = depart;
            oldSmile  = smile;
        }
            
    }
    fn _buildcabs(&mut self,frame: &mut canvas::Frame)
    {
        for m in (0..=self.timescale).step_by(self.timeinterval as usize) {
            let mx = (self.labelsize as f64) + ( ( ((m as f64) / (self.timeinterval as f64)) * 20.0)) + 4.0;
            let lw = if (m % 60) == 0 {2} else {1};
            let line = canvas::Path::line(
                               Point::new(mx as f32,self.topofcabs as f32),
                               Point::new(mx as f32,self.bottomofcabs as f32));
            let stroke = Stroke::default()
                            .with_color(Color::BLACK)
                            .with_width(lw as f32);
            frame.stroke(&line,stroke);
        }
        let r = self.labelsize as f64 + (((self.timescale as f64 / 
                                            self.timeinterval as f64) * 20.0));
        let line = canvas::Path::line(
                                Point::new(self.labelsize as f32,
                                           self.topofcabs as f32),
                                Point::new(r as f32,
                                           self.topofcabs as f32));
        let stroke = Stroke::default().with_color(Color::BLACK) 
                                      .with_width(2.0);
        frame.stroke(&line,stroke);
        let line = canvas::Path::line(
                                Point::new(self.labelsize as f32,
                                           self.bottomofcabs as f32),
                                Point::new(r as f32,
                                           self.bottomofcabs as f32));
        frame.stroke(&line,stroke);
    }
    fn _buildchart(&mut self,frame: &mut canvas::Frame)
    {
        for m in (0..=self.timescale).step_by(self.timeinterval as usize) {
            let mx = (self.labelsize as f64) + ( ( ((m as f64) / (self.timeinterval as f64)) * 20.0)) + 4.0;
            let lw = if (m % 60) == 0 {2} else {1};
            let line = canvas::Path::line(
                               Point::new(mx as f32,self.topofchart as f32),
                               Point::new(mx as f32,self.bottomofchart as f32));
            let stroke = Stroke::default()
                            .with_color(Color::BLACK)
                            .with_width(lw as f32);
            frame.stroke(&line,stroke);
        }
        let r = self.labelsize as f64 + (((self.timescale as f64 / 
                                            self.timeinterval as f64) * 20.0));
        let line = canvas::Path::line(
                                Point::new(self.labelsize as f32,
                                           self.topofchart as f32),
                                Point::new(r as f32,
                                           self.topofchart as f32));
        let stroke = Stroke::default().with_color(Color::BLACK) 
                                      .with_width(2.0);
        frame.stroke(&line,stroke);
        let line = canvas::Path::line(
                                Point::new(self.labelsize as f32,
                                           self.bottomofchart as f32),
                                Point::new(r as f32,
                                           self.bottomofchart as f32));
        frame.stroke(&line,stroke);
    }
    fn _buildStorageTracks(&mut self,frame: &mut canvas::Frame)
    {
        for m in (0..=self.timescale).step_by(self.timeinterval as usize) {
            let mx = (self.labelsize as f64) + ( ( ((m as f64) / (self.timeinterval as f64)) * 20.0)) + 4.0;
            let lw = if (m % 60) == 0 {2} else {1};
            let line = canvas::Path::line(
                               Point::new(mx as f32,self.topofstorage as f32),
                               Point::new(mx as f32,self.bottomofstorage as f32));
            let stroke = Stroke::default()
                            .with_color(Color::BLACK)
                            .with_width(lw as f32);
            frame.stroke(&line,stroke);
        }
        let r = self.labelsize as f64 + (((self.timescale as f64 / 
                                            self.timeinterval as f64) * 20.0));
        let line = canvas::Path::line(
                                Point::new(self.labelsize as f32,
                                           self.topofstorage as f32),
                                Point::new(r as f32,
                                           self.topofstorage as f32));
        let stroke = Stroke::default().with_color(Color::BLACK) 
                                      .with_width(2.0);
        frame.stroke(&line,stroke);
        let line = canvas::Path::line(
                                Point::new(self.labelsize as f32,
                                           self.bottomofstorage as f32),
                                Point::new(r as f32,
                                           self.bottomofstorage as f32));
        frame.stroke(&line,stroke);
    }
    fn buildWholeChart(&mut self, renderer: &Renderer, 
                        timetable: &TimeTableSystem) -> Vec<canvas::Geometry> {
        self.deleteWholeChart();
        self.timescale = timetable.TimeScale();
        self.timeinterval = timetable.TimeInterval();
        self.totallength = timetable.TotalLength();
        self.chartheight = (self.totallength * 20.0) + 40.0;
        let numIncrs =  (((self.timescale as f64) + (self.timeinterval as f64)) / 
                            (self.timeinterval as f64)) as i32;
        let cwidth = (numIncrs * 20) + self.labelsize as i32 + 20;
        let framewidth = cwidth as f32;
        let topOff = self.lheight;
        self.topofcabs = (topOff + 10.0) as f64;
        self.cabheight = 4.0 +timetable.NumberOfCabs() as f64 * self.lheight;
        self.bottomofcabs = self.topofcabs + self.cabheight;        
        self.topofchart = self.bottomofcabs + 10.0;
        self.bottomofchart = self.topofchart + self.chartheight;
        let topOff = self.topofchart + self.chartheight;
        self.topofstorage = topOff as f64 + 10.0;
        self.storageoffset = self.topofstorage;
        let topOff = self.topofstorage;
        let mut numstorage = 0;
        for station in timetable.StationsIter() {
            for storage in station.storagetracks() {
                numstorage += 1;
            }
        }
        self.storagetrackheight = (self.lheight) + 
                                    (numstorage as f64 * self.lheight);
        self.bottomofstorage = self.topofstorage + self.storagetrackheight;
        let cheight = self.bottomofstorage + self.lheight;
        let frameheight = cheight as f32;

        //eprintln!("*** ChartDisplay::buildWholeChart frame size is ({},{})",framewidth,frameheight);
        let mut frame = canvas::Frame::new(renderer, Size::new(framewidth, frameheight));

        self._buildTimeLine(&mut frame);
        self._buildcabs(&mut frame);
        let mut cabindex = 0;
        for (name, cab) in timetable.CabsIter() {
            self.addACab(&mut frame,cab,cabindex);
            cabindex += 1;
        }
        self._buildchart(&mut frame);
        self._buildStorageTracks(&mut frame);

        let mut sindex = 0;
        for station in timetable.StationsIter() {
            self.addAStation(&mut frame,station,sindex);
            sindex += 1;
        }
        for (number,train) in timetable.TrainsIter() {
            self.addATrain(&mut frame,timetable,train);
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

#[derive(Clone)]
pub struct TimeTableGUI {
    panes: pane_grid::State<Pane>,
    timetable: TimeTableSystem,
}

impl TimeTableGUI {
    pub fn new() -> Self {
        InitColorMap();
        let (mut panes, main) = pane_grid::State::new(Pane::Main);
        let Some((buttonpane,split)) = panes.split(Axis::Vertical,main,Pane::Buttons) else {panic!("failed to create buttons");};
        panes.resize(split,0.7490561);
        TimeTableGUI {
            panes, 
            timetable: TimeTableSystem::new(String::from("Dummy"),1440,15),
        }
    }
    pub fn update(&mut self, message: Message) -> Option<Action> {
        eprintln!("*** TimeTableGUI::update(): message is {:?}",message);
        match message {
            Message::FileSelected(File::Open) => {
                Some(Action::OpenFileSelect(".tt".to_string(),".".to_string(),
                                  "timetable.tt".to_string(),
                                  "Name of time table file to load".to_string()))
//                match TimeTableSystem::old("LJandBS.tt") {
//                    Ok(tt) => {self.timetable = tt;
//                                eprintln!("{} loaded: {}","LJandBS.tt",
//                                            self.timetable.Name());
//                    },
//                    Err(err) => {eprintln!("Count not open LJandBS.tt: {}",
//                                                err.to_string());
//                    },
//                }
//                None
            },
            Message::FileSelected(File::Exit) => exit(0),
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                //eprintln!("*** TimeTableGUI::update(): split is {:?}",split);
                self.panes.resize(split, ratio);
                None
            }
            _ => None,

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
    pub fn view(&self) -> Element<'_, Message> {

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
                            ).direction(scrollable::Direction::Both {
                                    horizontal: scrollable::Scrollbar::new(),
                                     vertical: scrollable::Scrollbar::new(),
                                })
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


//pub fn main() -> iced::Result {
//    InitColorMap();
//    iced::application("Time Table 2", TimeTableGUI::update, TimeTableGUI::view)
//        .theme(|_| Theme::Light)
//        .centered()
//        .run()
//}

pub type Manager = TimeTableGUI;

