# FAT-like EEPROM Filesystem Implementation Guide

## Overview
This guide describes how to implement a simplified FAT-like filesystem for EEPROM. The design is optimized for:
- Limited EEPROM space
- Wear leveling
- Simple implementation
- Easy understanding

## Filesystem Layout

### 1. Boot Sector (First 32 bytes)
```
Offset  Size    Description
0x00    2       Magic number (0xEE55)
0x02    1       Version (0x01)
0x03    2       Total sectors
0x05    2       Sectors per cluster
0x07    2       Root directory entries
0x09    2       Reserved sectors
0x0B    2       FAT size in sectors
0x0D    2       Root directory sector
0x0F    17      Reserved for future use
```

### 2. FAT (File Allocation Table)
- Each entry is 2 bytes
- Entry values:
  - 0x0000: Free cluster
  - 0xFFFF: End of file
  - 0x0001-0xFFFE: Next cluster number

### 3. Root Directory
Each directory entry (32 bytes):
```
Offset  Size    Description
0x00    8       Filename
0x08    3       Extension
0x0B    1       Attributes
0x0C    2       Reserved
0x0E    2       Creation time
0x10    2       Creation date
0x12    2       Last access date
0x14    2       Reserved
0x16    2       Last write time
0x18    2       Last write date
0x1A    2       Starting cluster
0x1C    4       File size
```

### 4. Data Area
- Clusters start after root directory
- Each cluster contains fixed-size data blocks
- Last cluster padded with zeros if needed

## Implementation Steps

### 1. Basic Structures

```rust
#[derive(Debug, Clone, Copy)]
pub struct BootSector {
    magic: u16,
    version: u8,
    total_sectors: u16,
    sectors_per_cluster: u16,
    root_dir_entries: u16,
    reserved_sectors: u16,
    fat_size: u16,
    root_dir_sector: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct DirEntry {
    filename: [u8; 8],
    extension: [u8; 3],
    attributes: u8,
    creation_time: u16,
    creation_date: u16,
    last_access_date: u16,
    last_write_time: u16,
    last_write_date: u16,
    starting_cluster: u16,
    file_size: u32,
}
```

### 2. Core Functions

```rust
impl EepromFs {
    // Initialize filesystem
    pub fn format(&mut self) -> Result<(), FsError> {
        // Write boot sector
        // Clear FAT
        // Initialize root directory
    }

    // File operations
    pub fn create_file(&mut self, name: &str) -> Result<File, FsError> {
        // Find free directory entry
        // Allocate clusters
        // Update FAT
    }

    pub fn read_file(&self, name: &str) -> Result<Vec<u8>, FsError> {
        // Find file in directory
        // Read clusters from FAT chain
        // Return data
    }

    pub fn write_file(&mut self, name: &str, data: &[u8]) -> Result<(), FsError> {
        // Find or create file
        // Allocate clusters if needed
        // Write data
        // Update FAT
    }

    // Directory operations
    pub fn list_directory(&self) -> Result<Vec<DirEntry>, FsError> {
        // Read directory entries
        // Filter out deleted entries
    }
}
```

### 3. Wear Leveling Implementation

```rust
impl EepromFs {
    // Track write cycles for each sector
    fn track_write_cycles(&mut self, sector: u16) -> Result<(), FsError> {
        // Read current write count
        // Increment counter
        // If threshold reached, rotate sector
    }

    // Rotate sector to new location
    fn rotate_sector(&mut self, old_sector: u16) -> Result<u16, FsError> {
        // Find least used sector
        // Copy data
        // Update FAT
        // Mark old sector as bad
    }
}
```

## Development Phases

### Phase 1: Basic Structure
1. Implement boot sector reading/writing
2. Create FAT management functions
3. Implement directory entry handling
4. Add basic file operations

### Phase 2: File Operations
1. Implement file creation
2. Add file reading
3. Add file writing
4. Implement file deletion

### Phase 3: Wear Leveling
1. Add write cycle tracking
2. Implement sector rotation
3. Add bad block management
4. Implement defragmentation

### Phase 4: Advanced Features
1. Add file attributes
2. Implement timestamps
3. Add file locking
4. Implement caching

## Testing Strategy

### 1. Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_sector() {
        // Test boot sector reading/writing
    }

    #[test]
    fn test_fat_operations() {
        // Test FAT chain operations
    }

    #[test]
    fn test_directory_operations() {
        // Test directory management
    }
}
```

### 2. Integration Tests
```rust
#[test]
fn test_file_operations() {
    // Test complete file operations
}

#[test]
fn test_wear_leveling() {
    // Test wear leveling mechanisms
}
```

## Usage Example

```rust
fn main() -> Result<(), FsError> {
    // Initialize filesystem
    let mut fs = EepromFs::new(eeprom_interface)?;
    
    // Format if needed
    if !fs.is_formatted()? {
        fs.format()?;
    }
    
    // Create and write file
    let data = b"Hello, EEPROM!";
    fs.write_file("test.txt", data)?;
    
    // Read file
    let content = fs.read_file("test.txt")?;
    println!("File content: {}", String::from_utf8_lossy(&content));
    
    Ok(())
}
```

## Tips for Implementation

1. **Start Simple**
   - Begin with basic read/write operations
   - Add features incrementally
   - Test thoroughly at each step

2. **Error Handling**
   - Use custom error types
   - Handle all error cases
   - Implement recovery mechanisms

3. **Wear Leveling**
   - Track write cycles
   - Rotate frequently written sectors
   - Implement bad block detection

4. **Testing**
   - Write tests before implementing features
   - Test edge cases
   - Simulate power failures

5. **Documentation**
   - Document all public APIs
   - Include usage examples
   - Explain design decisions

## Next Steps

1. Set up project structure
2. Implement boot sector handling
3. Add FAT management
4. Implement directory operations
5. Add file operations
6. Implement wear leveling
7. Add tests
8. Document code 