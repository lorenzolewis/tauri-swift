import SwiftRs
import AppKit

class Volume: NSObject {
    var name: SRString
    var path: SRString
    var total_capacity: Int
    var available_capacity: Int
    var is_removable: Bool
    var is_ejectable: Bool
    var is_root_filesystem: Bool
    
    public init(name: String, path: String, total_capacity: Int, available_capacity: Int, is_removable: Bool, is_ejectable: Bool, is_root_filesystem: Bool) {
        self.name = SRString(name);
        self.path = SRString(path);
        self.total_capacity = total_capacity
        self.available_capacity = available_capacity
        self.is_removable = is_removable
        self.is_ejectable = is_ejectable
        self.is_root_filesystem = is_root_filesystem
    }
}

@_cdecl("get_mounts")
func getMounts() -> SRObjectArray {
    // let keys: [URLResourceKey] = [
    //     .volumeNameKey,
    //     .volumeIsRemovableKey,
    //     .volumeIsEjectableKey,
    //     .volumeTotalCapacityKey,
    //     .volumeAvailableCapacityKey,
    //     .volumeIsRootFileSystemKey,
    // ]
    
    // let paths = autoreleasepool {
    //     FileManager().mountedVolumeURLs(includingResourceValuesForKeys: keys, options: [])
    // }
    
    // var validMounts: [Volume] = []
        
    // if let urls = paths {
    //     autoreleasepool {
    //         for url in urls {
    //             let components = url.pathComponents
    //             if components.count == 1 || components.count > 1
    //                && components[1] == "Volumes"
    //             {
    //                 let metadata = try? url.promisedItemResourceValues(forKeys: Set(keys))
                    
    //                 let volume = Volume(
    //                     name: metadata?.volumeName ?? "",
    //                     path: url.path,
    //                     total_capacity: metadata?.volumeTotalCapacity ?? 0,
    //                     available_capacity: metadata?.volumeAvailableCapacity ?? 0,
    //                     is_removable: metadata?.volumeIsRemovable ?? false,
    //                     is_ejectable: metadata?.volumeIsEjectable ?? false,
    //                     is_root_filesystem: metadata?.volumeIsRootFileSystem ?? false
    //                 )
                    
                    
    //                 validMounts.append(volume)
    //             }
    //         }
    //     }
    // }
    
    return SRObjectArray([Volume(name: "Value", path: "value", total_capacity: 5, available_capacity: 5, is_removable: true, is_ejectable: true, is_root_filesystem: true)])
}

class Test: NSObject {
    var null: Bool
    
    public init(_ null: Bool)
    {
        self.null = null;
    }
}

@_cdecl("return_nullable")
func returnNullable(null: Bool) -> Bool {  
    return null
}