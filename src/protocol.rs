#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::mem::{self, size_of};
use std::io::{Cursor,Write};


pub static CHANNEL_CHRONICLE_REQ:   &str = "SOME.d3fs_master.metainfo_chronicle.qxdmlogger.request";
pub static CHANNEL_CHRONICLE_RES:   &str = "SOME.d3fs_master.metainfo_chronicle.qxdmlogger.response";
pub static CHANNEL_FILE_REQ:        &str = "SOME.d3fs_master.metainfo_file.qxdmlogger.request";
pub static CHANNEL_FILE_RES:        &str = "SOME.d3fs_master.metainfo_file.qxdmlogger.response";
pub static CHANNEL_FILE_EVENT:      &str = "SOME.qxdmlogger.file_status.notification";
pub static CHANNEL_HEARTBEAT:       &str = "SOME.qxdmlogger.heartbeat.notification";
pub static CHANNEL_DISK_CTRL:       &str = "SOME.d3fs_master.disk_ctrl.notification";
pub static CHANNEL_SHUTDOWN:        &str ="SOME.dim_daemon.shutdown.notification";

pub static CLIENT_ID: u16 = 0x0726;

// 
// header
// 

#[repr(align(1))]
#[derive(Debug)]
pub struct DapHeader{
    messageId:          u32,            // uint16_t serviceId; uint16_t methodId
    length:             u32,            // 
    requestId:          u32,            // uint16_t clientId; uint16_t sessionId
    protocolVersion:    u8,             // header version
    interfaceVersion:   u8,             // contains the Major Version of the Service Interface
    messageType:        u8,             // distinguish different request type or response/error
    returnCode:         u8,             // succeed or specific error code       
} 

// 
// payload 
// 

#[repr(align(1))]
#[derive(Debug)]
pub struct ChronicleReqPayload {
    pub logger:         u8,
    pub portType:       u8,
    pub portId:         u8,
    pub reserved:       u8
}
#[repr(align(1))]
#[derive(Debug)]
pub struct ChronicleResPayload {
    pub chronicleId:    [u8; 36],
    pub portType:       u8,
    pub portId:         u8,
    pub slotId:         u8,
    pub reserved:       u8,
}
#[repr(align(1))]
#[derive(Debug)]
pub struct FileReqPayload {
    pub chronicleId:    [u8; 36],
    pub fileType:       u32,
    pub sizeName:       u32,
    pub sizeExt:        u32,
    pub fileName:       Vec<u8>,
    pub fileExt:        Vec<u8>,
}
#[repr(align(1))]
#[derive(Debug)]
pub struct FileResPayload{
    pub fileId:         [u8; 36],           //unique
    pub portType:       u8,
    pub portId:         u8,                
    pub slotId:         u8,
    pub reserved:       u8,
    pub capacity:       u32,
    pub sizeDir:        u32,
    pub sizePath:       u32,
    pub sizeAbs:        u32,
    pub dirPath:        Vec<u8>,            // char* : /media/ssd1/DL_UUID
    pub filePath:       Vec<u8>,            // char* : chronicle_id/file_name.file_ext
    pub absolutePath:   Vec<u8>,            // char* : /media/ssd1/DL_UUID/chronicle_id/file_name.file_ext
}
#[repr(align(1))]
#[derive(Debug)]
pub struct FileStatusPayload{
    pub fileId:         [u8; 36],
    pub opCode:         u32,                // operation code
    pub timestamp:      u32,                // trace timeline, second based
    pub fileRealSize:   u32,    	        // live file size
}
#[repr(align(1))]
#[derive(Debug)]
pub struct DiskCtrlPayload{
    pub slotId:         u8,
    pub opCode:         u8,                 // operation code 和filestatus的区分开
    pub reserved:       u16,          
}

//
// frame
//

#[repr(align(1))]
#[derive(Debug)]
pub struct  ChronicleReqMsg{
    pub header:     DapHeader,
    pub payload:    ChronicleReqPayload
}
#[repr(align(1))]
#[derive(Debug)]
pub struct  ChronicleResMsg{
    pub header:     DapHeader,
    pub payload:    ChronicleResPayload
}
#[repr(align(1))]
#[derive(Debug)]
pub struct  FileReqMsg{
    pub header:     DapHeader,
    pub payload:    FileReqPayload
}
#[repr(align(1))]
#[derive(Debug)]
pub struct  FileResMsg{
    pub header:     DapHeader,
    pub payload:    FileResPayload
}
#[repr(align(1))]
#[derive(Debug)]
pub struct  FileStatusMsg{
    pub header:     DapHeader,
    pub payload:    FileStatusPayload
}
#[repr(align(1))]
#[derive(Debug)]
pub struct  DiskCtrlMsg{
    pub header:     DapHeader,
    pub payload:    DiskCtrlPayload
}

//
// utils
//

#[allow(dead_code)]
enum MessageType {
    Request         = 0x00,                 // A request expecting a response (even void)
    RequestNoReturn = 0x01,                 // fire & forget
    Notification    = 0x02,                 // a notification/event callback expecting no response
	Response        = 0x80,			        // response message
    Error           = 0x81                  // response containing an error
}
#[allow(dead_code)]
enum ReturnCode {
    OK              = 0x00,
    NotOK           = 0x01
}
pub enum OpCode {
    New             = 1,
    End             = 2,
    Update          = 3
}

// 
// 对应函数
// 

// impl ChronicleReqMsg {
//     pub fn form_dap_msg(payload: ChronicleReqPayload) -> Self {
//         let serviceId: u16 =  0x6351;
//         let methodId: u16 = 0x1001;
//         let messageId: u32 = ((serviceId as u32) << 16) | (methodId as u32);

//         let length = mem::size_of::<ChronicleReqPayload>() + mem::size_of::<DapHeader>() - 8;
//         let length = length as u32;

//         let sessionId: u16 = get_session_id();
//         let requestId: u32 = ((CLIENT_ID as u32) << 16 ) | (sessionId as u32);

//         let messageType = MessageType::Request as u8;

//         let returnCode = ReturnCode::OK as u8;


//         let header:DapHeader = DapHeader { 
//             messageId,   
//             length, 
//             requestId, 
//             protocolVersion: 0, 
//             interfaceVersion: 0, 
//             messageType, 
//             returnCode
//         };
//         ChronicleReqMsg {header, payload}
//     }

//     pub fn to_dap_msg_bin(&self) -> Vec<u8> {
//         let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

//         buffer.write_u32::<BigEndian>(self.header.messageId).unwrap();
//         buffer.write_u32::<BigEndian>(self.header.length).unwrap();
//         buffer.write_u32::<BigEndian>(self.header.requestId).unwrap();
//         buffer.write_u8(self.header.protocolVersion).unwrap();
//         buffer.write_u8(self.header.interfaceVersion).unwrap();
//         buffer.write_u8(self.header.messageType).unwrap();
//         buffer.write_u8(self.header.returnCode).unwrap();

//         buffer.write_u8(self.payload.logger).unwrap();
//         buffer.write_u8(self.payload.portType).unwrap();
//         buffer.write_u8(self.payload.portId).unwrap();
//         buffer.write_u8(self.payload.reserved).unwrap();

//         buffer.into_inner()
//     }

// }

// impl FileReqMsg {
//     pub fn form_dap_msg(payload: FileReqPayload) -> Self {
//         let serviceId: u16 =  0x6351;
//         let methodId: u16 = 0x1002;
//         let messageId: u32 = ((serviceId as u32) << 16) | (methodId as u32);

//         let mut length =mem::size_of::<DapHeader>() - 8;
//         length += 36 + 4 + 4 + 4 + payload.fileName.len() + payload.fileExt.len();
//         let length = length as u32;

//         let sessionId: u16 = get_session_id();
//         let requestId: u32 = ((CLIENT_ID as u32) << 16 ) | (sessionId as u32);

//         let messageType = MessageType::Request as u8;

//         let returnCode = ReturnCode::OK as u8;


//         let header:DapHeader = DapHeader { 
//             messageId,   
//             length, 
//             requestId, 
//             protocolVersion: 0, 
//             interfaceVersion: 0, 
//             messageType, 
//             returnCode
//         };
//         FileReqMsg {header, payload}
//     }

//     pub fn to_dap_msg_bin(&self) -> Vec<u8> {
//         let mut buffer = Cursor::new(Vec::new());

//         buffer.write_u32::<BigEndian>(self.header.messageId).unwrap();
//         buffer.write_u32::<BigEndian>(self.header.length).unwrap();
//         buffer.write_u32::<BigEndian>(self.header.requestId).unwrap();
//         buffer.write_u8(self.header.protocolVersion).unwrap();
//         buffer.write_u8(self.header.interfaceVersion).unwrap();
//         buffer.write_u8(self.header.messageType).unwrap();
//         buffer.write_u8(self.header.returnCode).unwrap();

//         buffer.write_all(&self.payload.chronicleId).unwrap();
//         buffer.write_u32::<LittleEndian>(self.payload.fileType).unwrap();
//         buffer.write_u32::<LittleEndian>(self.payload.sizeName).unwrap();
//         buffer.write_u32::<LittleEndian>(self.payload.sizeExt).unwrap();
//         buffer.write_all(&self.payload.fileName).unwrap();
//         buffer.write_all(&self.payload.fileExt).unwrap();

//         buffer.into_inner()
//     }

// }

// impl ChronicleResMsg {
//     pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
//         if bytes.len() < size_of::<Self>() {
//             error!("data from channel is not correct");
//             return Option::None;
//         }
//         // let mut res: ChronicleResMsg = unsafe { 
//         //     mem::zeroed() 
//         // };
//         // let bytes_ptr = bytes.as_ptr() as *const Self;
//         // unsafe {
//         //     std::ptr::copy_nonoverlapping(bytes_ptr, &mut res, 1);
//         // }
        
//         let mut offset = 0;

//         let messageId = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         let length = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         let requestId = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;

//         let protocolVersion = bytes[offset];
//         offset += 1;
//         let interfaceVersion = bytes[offset];
//         offset += 1;
//         let messageType = bytes[offset];
//         offset += 1;
//         let returnCode = bytes[offset];
//         offset += 1;

//         let header = DapHeader {
//             messageId,
//             length,
//             requestId,
//             protocolVersion,
//             interfaceVersion,
//             messageType,
//             returnCode,
//         };

//         let mut chronicleId = [0u8; 36];
//         chronicleId.copy_from_slice(&bytes[offset..offset+36]);
//         offset += 36;
//         let portType = bytes[offset];
//         offset += 1;
//         let portId = bytes[offset];
//         offset += 1;
//         let slotId = bytes[offset];
//         offset += 1;
//         let reserved = bytes[offset];

//         let payload = ChronicleResPayload {
//             chronicleId,
//             portType,
//             portId,
//             slotId,
//             reserved,
//         };

//         let res = ChronicleResMsg {
//             header,
//             payload,
//         };

//         // info!("chronicle respond from d3fs:{:?}",res);

//         Option::Some(res)
        
//     }
// }

// impl FileResMsg {
//     pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
//         let mut offset = 0;

//         let messageId = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         let length = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         let requestId = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;

//         let protocolVersion = bytes[offset];
//         offset += 1;
//         let interfaceVersion = bytes[offset];
//         offset += 1;
//         let messageType = bytes[offset];
//         offset += 1;
//         let returnCode = bytes[offset];
//         offset += 1;

//         let header = DapHeader {
//             messageId,
//             length,
//             requestId,
//             protocolVersion,
//             interfaceVersion,
//             messageType,
//             returnCode,
//         };

//         let mut fileId = [0u8; 36];
//         fileId.copy_from_slice(&bytes[offset..offset+36]);
//         offset += 36;
//         // info!("fileId {:?}",fileId);

//         let portType = bytes[offset];
//         offset += 1;
//         // info!("portType {:?}",portType);

//         let portId = bytes[offset];
//         offset += 1;
//         // info!("portId {:?}",portId);

//         let slotId = bytes[offset];
//         offset += 1;
//         // info!("slotId {:?}",slotId);

//         let reserved = bytes[offset];
//         offset += 1;
//         // info!("reserved {:?}",reserved);

//         let capacity = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         // info!("capacity {:?}",capacity);

//         let sizeDir = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         // info!("sizeDir {:?}",sizeDir);

//         let sizePath = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         // info!("sizePath {:?}",sizePath);

//         let sizeAbs = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         // info!("sizeAbs {:?}",sizeAbs);

//         let mut dirPath = vec![0u8; sizeDir as usize];
//         dirPath.copy_from_slice(&bytes[offset..offset+(sizeDir as usize)]);
//         offset += sizeDir as usize;
//         // info!("dirPath: {:?}", dirPath);
//         match std::str::from_utf8(&dirPath) {
//             Ok(string_path) => debug!("dirPath: {}", string_path),
//             Err(_) => error!("fail dirPath UTF-8 data to str"),
//         }

//         let mut filePath = vec![0u8; sizePath as usize];
//         filePath.copy_from_slice(&bytes[offset..offset+(sizePath as usize)]);
//         offset += sizePath as usize;
//         // info!("filePath: {:?}", filePath);
//         match std::str::from_utf8(&filePath) {
//             Ok(string_path) => debug!("filePath: {}", string_path),
//             Err(_) => error!("fail filePath UTF-8 data to str"),
//         }

//         let mut absolutePath = vec![0u8; sizeAbs as usize];
//         absolutePath.copy_from_slice(&bytes[offset..offset+(sizeAbs as usize)]);
//         // info!("absolutePath: {:?}", absolutePath);
//         match std::str::from_utf8(&absolutePath) {
//             Ok(string_path) => debug!("absolutePath: {}", string_path),
//             Err(_) => error!("fail absolutePath UTF-8 data to str"),
//         }
//         // offset += 2048;

//         let payload: FileResPayload = FileResPayload {
//             fileId,
//             portType,
//             portId,
//             slotId,
//             reserved,
//             capacity,
//             sizeDir,
//             sizePath,
//             sizeAbs,
//             dirPath,      
//             filePath,        
//             absolutePath,    
//         };

//         let res = FileResMsg {
//             header,
//             payload,
//         };

//         // info!("file respond from d3fs:{:?}",res);

//         Option::Some(res)
//     }
// }

// impl FileStatusMsg {
//     pub fn form_dap_msg(payload: FileStatusPayload) -> Self {
//         let serviceId: u16 =  0x6351;
//         let methodId: u16 = 0x1003;
//         let messageId: u32 = ((serviceId as u32) << 16) | (methodId as u32);

//         let mut length =mem::size_of::<DapHeader>() - 8;
//         length += 36 + 4 + 4 + 4;
//         let length: u32 = length as u32;

//         let sessionId: u16 = get_session_id();
//         let requestId: u32 = ((CLIENT_ID as u32) << 16 ) | (sessionId as u32);

//         let messageType = MessageType::Request as u8;

//         let returnCode = ReturnCode::OK as u8;


//         let header:DapHeader = DapHeader { 
//             messageId,   
//             length, 
//             requestId, 
//             protocolVersion: 0, 
//             interfaceVersion: 0, 
//             messageType, 
//             returnCode
//         };
//         FileStatusMsg {header, payload}
//     }

//     pub fn to_dap_msg_bin(&self) -> Vec<u8> {
//         let mut buffer = Cursor::new(Vec::new());

//         buffer.write_u32::<BigEndian>(self.header.messageId).unwrap();
//         buffer.write_u32::<BigEndian>(self.header.length).unwrap();
//         buffer.write_u32::<BigEndian>(self.header.requestId).unwrap();
//         buffer.write_u8(self.header.protocolVersion).unwrap();
//         buffer.write_u8(self.header.interfaceVersion).unwrap();
//         buffer.write_u8(self.header.messageType).unwrap();
//         buffer.write_u8(self.header.returnCode).unwrap();

//         buffer.write_all(&self.payload.fileId).unwrap();
//         buffer.write_u32::<LittleEndian>(self.payload.opCode).unwrap();
//         buffer.write_u32::<LittleEndian>(self.payload.timestamp).unwrap();
//         buffer.write_u32::<LittleEndian>(self.payload.fileRealSize).unwrap();

//         buffer.into_inner()
//     }
// }

// impl DiskCtrlMsg {
//     pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
//         if bytes.len() < size_of::<Self>() {
//             error!("data from channel is not correct");
//             return Option::None;
//         }
        
//         let mut offset = 0;

//         let messageId = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         let length = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;
//         let requestId = u32::from_be_bytes(bytes[offset..offset+4].try_into().unwrap());
//         offset += 4;

//         let protocolVersion = bytes[offset];
//         offset += 1;
//         let interfaceVersion = bytes[offset];
//         offset += 1;
//         let messageType = bytes[offset];
//         offset += 1;
//         let returnCode = bytes[offset];
//         offset += 1;

//         let header = DapHeader {
//             messageId,
//             length,
//             requestId,
//             protocolVersion,
//             interfaceVersion,
//             messageType,
//             returnCode,
//         };


//         let slotId = bytes[offset];
//         offset += 1;
//         let opCode = bytes[offset];
//         offset += 1;
//         let reserved = u16::from_be_bytes(bytes[offset..offset+2].try_into().unwrap());
//         // offset += 2;

//         let payload = DiskCtrlPayload {
//             slotId,
//             opCode,
//             reserved,
//         };

//         let res = DiskCtrlMsg {
//             header,
//             payload,
//         };

//         info!("chronicle respond from d3fs:{:?}",res);

//         Option::Some(res)
        
//     }
// }