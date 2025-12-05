use super::loaded_stages::{LoadedStage, LoadedStages};
use super::map_zones::MapZones;
use super::monsters::LargeMonsters;
use super::offsets::{GEN_QUEST_PROP_PRT, MAIN_QUEST_PROP_PRT};
use super::quest_end_flag::QuestEndFlag;
use super::quest_string::QuestStrings;
use super::reward::Rewards;
use super::supply_items::SupplyItems;
use crate::editor::quest::header::{MapInfo, QuestFileHeader};
use crate::editor::quest::quest_type_flags::{GenQuestProp, QuestTypeFlags};
use better_cursor::{BetterRead, BetterSeek, BetterWrite};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Cursor, Result};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QuestFile {
    pub header: QuestFileHeader,
    pub gen_quest_prop: GenQuestProp,
    pub quest_type_flags: QuestTypeFlags,
    pub map_info: MapInfo,
    pub map_zones: MapZones,
    pub large_monsters: LargeMonsters,
    pub rewards: Rewards,
    pub loaded_stages: LoadedStages,
    pub unk_data: Vec<u8>,
    pub supply_items: SupplyItems,
    pub strings: QuestStrings,
}

impl QuestFile {
    pub fn from_path(filename: &str) -> Result<QuestFile> {
        let mut reader = better_cursor::from_filepath(filename)?;

        return  QuestFile::from_reader(&mut reader);
    }

    pub fn from_reader<R: BetterRead>(reader: &mut R) -> Result<QuestFile> {
        let header = reader.read_struct::<QuestFileHeader>()?;

        reader.seek_start(GEN_QUEST_PROP_PRT as u64)?;
        let gen_quest_prop = reader.read_struct::<GenQuestProp>()?;

        reader.seek_start(MAIN_QUEST_PROP_PRT as u64)?;
        let quest_type_flags = reader.read_struct::<QuestTypeFlags>()?;

        let unk_data: Vec<u8> = reader.read_buffer(115)?;

        reader.seek_start(header.map_info as u64)?;
        let map_info = reader.read_struct::<MapInfo>()?;

        reader.seek_start(header.loaded_stages_ptr as u64)?;
        let mut loaded_stages: Vec<LoadedStage> = vec![];
        while reader.current_position()? != header.fixed_cords1 as u64 {
            let loaded_stage = reader.read_struct::<LoadedStage>()?;
            loaded_stages.push(loaded_stage);
        }

        reader.seek_start(header.large_monster_ptr as u64)?;
        let large_monsters = reader.read_custom::<LargeMonsters>()?;

        reader.seek_start(header.reward_ptr as u64)?;
        let rewards: Rewards = reader.read_custom::<Rewards>()?;

        reader.seek_start(header.quest_area_ptr as u64)?;
        let map_zones = reader.read_custom::<MapZones>()?;

        reader.seek_start(header.supply_box_ptr as u64)?;
        let supply_items = reader.read_custom::<SupplyItems>()?;

        let strings = QuestStrings::from_reader(
            reader,
            quest_type_flags.main_quest_prop.quest_strings_ptr,
            None,
        )?;

        Ok(QuestFile {
            header,
            gen_quest_prop,
            quest_type_flags,
            map_info,
            map_zones,
            large_monsters,
            rewards,
            supply_items,
            loaded_stages,
            unk_data,
            strings,
        })
    }

    pub fn save_file(filename: &str, quest: &mut QuestFile) -> Result<()> {
        let original = QuestFile::from_path(filename)?;
        let end_flag = QuestEndFlag::from_path(filename)?;
        let mut writer = better_cursor::from_filepath_write(filename)?;

        QuestFile::remove_file_extra_data(&mut writer, &end_flag);

        QuestFile::write(&mut writer, &original, quest)?;

        Ok(())
    }

    pub fn save_buffer(writer: &mut Cursor<Vec<u8>>, quest: &mut QuestFile) -> Result<()> {
        let original = QuestFile::from_reader(writer)?;
        let end_flag = QuestEndFlag::from_reader(writer)?;
        writer.seek_start(0)?;

        QuestFile::remove_buffer_extra_data(writer, &end_flag);

        QuestFile::write(writer, &original, quest)?;

        Ok(())
    }

    pub fn write<W: BetterWrite>(writer: &mut W, original: &QuestFile, quest: &mut QuestFile) -> Result<()> {
        writer.write_struct_on(&mut quest.gen_quest_prop, GEN_QUEST_PROP_PRT as u64)?;
        writer.write_struct_on(&mut quest.quest_type_flags, MAIN_QUEST_PROP_PRT as u64)?;

        writer.seek_start(original.header.loaded_stages_ptr as u64)?;
        for loaded_stage in &mut quest.loaded_stages {
            if writer.current_position()? == original.header.fixed_cords1 as u64 {
                break;
            }

            writer.write_struct(loaded_stage)?;
        }

        // Write mapinfo
        writer.seek_start(quest.header.map_info as u64)?;
        writer.write_struct(&mut quest.map_info)?;

        // Write supply items
        writer.seek_start(quest.header.supply_box_ptr as u64)?;
        writer.write_custom(&mut quest.supply_items)?;

        QuestFile::write_extra_data(writer, quest)?;

        Ok(())
    }

    pub fn write_extra_data<W: BetterWrite>(
        writer: &mut W,
        quest: &mut QuestFile,
    ) -> Result<()> {
        let mut new_end_flag = QuestEndFlag::new(writer.len()? as u32);

        writer.seek_start(new_end_flag.start_ptr as u64)?;

        quest.header.reward_ptr = writer.write_custom(&mut quest.rewards)? as u32;
        quest.quest_type_flags.main_quest_prop.quest_strings_ptr =
            writer.write_custom(&mut quest.strings)? as u32;
        quest.header.quest_area_ptr = writer.write_custom(&mut quest.map_zones)? as u32;

        quest
            .large_monsters
            .large_monster_pointers
            .large_monster_ids = quest.large_monsters.write_monster_ids(writer)? as u32;
        quest
            .large_monsters
            .large_monster_pointers
            .large_monster_spawns = quest.large_monsters.write_monster_spawns(writer)? as u32;
        writer.write_struct_on(
            &mut quest.large_monsters.large_monster_pointers,
            quest.header.large_monster_ptr as u64,
        )?;
        writer.write_struct_on(&mut quest.header, 0)?;
        writer.write_struct_on(&mut quest.quest_type_flags, MAIN_QUEST_PROP_PRT as u64)?;

        writer.write_struct(&mut new_end_flag)?;
        writer.write_u8(&0)?;

        Ok(())
    }

    fn remove_file_extra_data(
        writer: &mut File,
        end_flag: &QuestEndFlag,
    ) -> Result<()> {
        let have_sign = end_flag.is_valid();
        if have_sign {
            writer.set_len(end_flag.start_ptr as u64)?;
        }

        Ok(())
    }

    fn remove_buffer_extra_data(
        writer: &mut Cursor<Vec<u8>>,
        end_flag: &QuestEndFlag,
    ) -> Result<()> {
        let have_sign = end_flag.is_valid();
        if have_sign {
            writer.get_mut().truncate(end_flag.start_ptr as usize);
        }

        Ok(())
    }
}
