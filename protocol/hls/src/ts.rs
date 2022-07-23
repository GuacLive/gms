use {
    super::errors::MediaError,
    bytes::BytesMut,
    std::{fs, fs::File, io::Write},
};

pub struct Ts {
    ts_number: u32,
    pts_number: u32,
    folder_name: String,
}

impl Ts {
    pub fn new(app_name: String, stream_name: String) -> Self {
        let folder_name = format!("./{}/{}", app_name, stream_name);
        fs::create_dir_all(folder_name.clone()).unwrap();

        Self {
            ts_number: 0,
            pts_number: 0,
            folder_name,
        }
    }
    pub fn write(&mut self, data: BytesMut, partial: bool) -> Result<(String, String), MediaError> {
        let ts_file_name = format!(
            "{}{}.ts",
            self.ts_number.clone(),
            if partial {
                self.pts_number += 1;
                format!(".{}", self.pts_number)
            } else {
                self.pts_number = 0;
                self.ts_number += 1;
                String::from("")
            },
        );
        let ts_file_path = format!("{}/{}", self.folder_name, ts_file_name);

        let mut ts_file_handler = File::create(ts_file_path.clone())?;
        ts_file_handler.write_all(&data[..])?;

        Ok((ts_file_name, ts_file_path))
    }
    pub fn delete(&mut self, ts_file_name: String) {
        fs::remove_file(ts_file_name).unwrap();
    }
}
