let wasm;
let isGenerating = false;

// Initialize the WebAssembly module
async function init() {
    try {
        // Import the wasm module - this path will need to be updated after building
        wasm = await import('../pkg/spirals.js');
        await wasm.default();
        
        // Display version info
        const versionInfo = document.getElementById('versionInfo');
        versionInfo.textContent = `WebAssembly module version: ${wasm.get_version()}`;
        
        console.log('WebAssembly module loaded successfully');
        
        // Enable the generate button
        document.getElementById('generateBtn').disabled = false;
    } catch (error) {
        showError(`Failed to load WebAssembly module: ${error.message}`);
        console.error('WebAssembly initialization error:', error);
    }
}

// Generate and display the spiral
function generateSpiral() {
    if (isGenerating || !wasm) return;
    
    isGenerating = true;
    const generateBtn = document.getElementById('generateBtn');
    const loadingStatus = document.getElementById('loadingStatus');
    const performanceInfo = document.getElementById('performanceInfo');
    const errorMessage = document.getElementById('errorMessage');
    
    // Get parameters from inputs
    const width = parseInt(document.getElementById('width').value);
    const height = parseInt(document.getElementById('height').value);
    const numPoints = parseInt(document.getElementById('numPoints').value);
    
    // Validate inputs
    if (width < 100 || width > 2000 || height < 100 || height > 2000) {
        showError('Width and height must be between 100 and 2000 pixels');
        isGenerating = false;
        return;
    }
    
    if (numPoints < 1000 || numPoints > 10000000) {
        showError('Number of points must be between 1,000 and 10,000,000');
        isGenerating = false;
        return;
    }
    
    // Update UI
    generateBtn.disabled = true;
    generateBtn.textContent = 'Generating...';
    loadingStatus.classList.remove('hidden');
    errorMessage.classList.add('hidden');
    performanceInfo.textContent = '';
    
    // Update canvas size
    const canvas = document.getElementById('spiralCanvas');
    canvas.width = width;
    canvas.height = height;
    
    try {
        const startTime = performance.now();
        
        // Generate spiral data using WebAssembly
        const pixelData = wasm.generate_spiral(width, height, numPoints);
        
        const wasmTime = performance.now() - startTime;
        
        // Render to canvas
        const renderStartTime = performance.now();
        renderToCanvas(canvas, pixelData, width, height);
        const renderTime = performance.now() - renderStartTime;
        
        const totalTime = performance.now() - startTime;
        
        // Update performance info
        performanceInfo.innerHTML = `
            WebAssembly generation: ${wasmTime.toFixed(2)}ms<br>
            Canvas rendering: ${renderTime.toFixed(2)}ms<br>
            Total time: ${totalTime.toFixed(2)}ms<br>
            Points processed: ${numPoints.toLocaleString()}
        `;
        
        // Enable download button
        document.getElementById('downloadBtn').disabled = false;
        
    } catch (error) {
        showError(`Generation failed: ${error.message}`);
        console.error('Generation error:', error);
    }
    
    // Reset UI
    isGenerating = false;
    generateBtn.disabled = false;
    generateBtn.textContent = 'Generate Spiral';
    loadingStatus.classList.add('hidden');
}

// Render pixel data to canvas
function renderToCanvas(canvas, pixelData, width, height) {
    const ctx = canvas.getContext('2d');
    
    // Create ImageData from the pixel data
    const imageData = ctx.createImageData(width, height);
    
    // Copy the pixel data to ImageData
    // Note: pixelData is a Uint8Array from WebAssembly
    const data = new Uint8Array(pixelData);
    imageData.data.set(data);
    
    // Draw the image data to the canvas
    ctx.putImageData(imageData, 0, 0);
}

// Download the canvas as an image
function downloadImage() {
    const canvas = document.getElementById('spiralCanvas');
    const link = document.createElement('a');
    link.download = `prime-spiral-${Date.now()}.png`;
    link.href = canvas.toDataURL();
    link.click();
}

// Show error message
function showError(message) {
    const errorElement = document.getElementById('errorMessage');
    errorElement.textContent = message;
    errorElement.classList.remove('hidden');
}

// Handle input changes to update canvas size preview
function updateCanvasSize() {
    const width = parseInt(document.getElementById('width').value) || 800;
    const height = parseInt(document.getElementById('height').value) || 800;
    
    const canvas = document.getElementById('spiralCanvas');
    if (!isGenerating) {
        canvas.width = width;
        canvas.height = height;
        
        // Clear canvas with black background
        const ctx = canvas.getContext('2d');
        ctx.fillStyle = 'black';
        ctx.fillRect(0, 0, width, height);
    }
}

// Add event listeners
document.addEventListener('DOMContentLoaded', () => {
    // Initialize WebAssembly
    init();
    
    // Add input event listeners for live canvas size updates
    document.getElementById('width').addEventListener('input', updateCanvasSize);
    document.getElementById('height').addEventListener('input', updateCanvasSize);
    
    // Handle Enter key in input fields
    document.querySelectorAll('input').forEach(input => {
        input.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                generateSpiral();
            }
        });
    });
    
    // Initialize canvas
    updateCanvasSize();
});

// Make functions available globally for onclick handlers
window.generateSpiral = generateSpiral;
window.downloadImage = downloadImage;