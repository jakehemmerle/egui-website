const fs = require('fs');
const path = require('path');

const distDir = path.join(__dirname, 'dist');
const indexHtmlPath = path.join(distDir, 'index.html');
const jsFilePath = fs.readdirSync(distDir).find(file => file.startsWith('egui_website') && file.endsWith('.js'));

if (jsFilePath) {
  let indexHtmlContent = fs.readFileSync(indexHtmlPath, 'utf8');
  indexHtmlContent = indexHtmlContent.replace(/egui_website.*\.js/, jsFilePath);
  fs.writeFileSync(indexHtmlPath, indexHtmlContent);
  console.log('Updated index.html with the correct egui_website.js filename');
} else {
  console.error('egui_website.js file not found in dist directory');
}
