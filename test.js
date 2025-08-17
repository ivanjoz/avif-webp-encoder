import { execSync } from 'child_process';
import path from 'path';
import fs from 'fs';

// Determine the project root, assuming the script is run from the directory
// that contains 'rust-src' and 'binaries' folders.
const projectRoot = process.cwd();

try {
  const rustSrcPath = path.join(projectRoot, 'rust-src');
  const binariesPath = path.join(projectRoot, 'binaries');
  const goProjectPath = path.join(projectRoot, 'imageconv');

  // Ensure the binaries directory exists
  if (!fs.existsSync(binariesPath)) {
    console.log(`Creating directory: ${binariesPath}`);
    fs.mkdirSync(binariesPath, { recursive: true });
  }

  // Determine the correct executable name based on the operating system
  const executableName = process.platform === 'win32' ? 'avif-converter.exe' : 'avif-converter';

  // Construct the full path to the built executable
  const builtExecutableSrcPath = path.join(rustSrcPath, 'target', 'release', executableName);

  // Construct the full path for the destination file
  const destinationPath = path.join(binariesPath, 'avif-converter-local-debug');

  console.log(`Entering ${rustSrcPath} and executing 'cargo build --release'...`);
  // Execute 'cargo build' within the 'rust-src' directory
  execSync('cargo build', { stdio: 'inherit', cwd: rustSrcPath });
  console.log('Cargo build completed successfully.');

  // Verify that the compiled executable exists
  if (!fs.existsSync(builtExecutableSrcPath)) {
    throw new Error(`Compiled executable not found at: ${builtExecutableSrcPath}`);
  }

  console.log(`Copying "${builtExecutableSrcPath}" to "${destinationPath}"...`);
  // Copy the compiled file to the binaries folder with the specified new name
  fs.copyFileSync(builtExecutableSrcPath, destinationPath);
  console.log('File copied successfully.');

  //Execute the go file
  console.log(`Executing ${goProjectPath}...`);
  execSync('go run tests.go debug', { stdio: 'inherit', cwd: goProjectPath });

} catch (error) {
  console.error(`Error during build and copy process: ${error.message}`);
  // Exit with a non-zero code to indicate script failure
  process.exit(1);
}
