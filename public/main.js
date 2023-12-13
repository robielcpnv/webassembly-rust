async function init() {
let rustApp = null;
try {
   rustApp = await import('../pkg');
} catch (err) {
   console.error(`Unexpected error in load rust app: ${err}`);
   return;
}

console.log('Rust app loaded', rustApp);

const input = document.getElementById('upload')
const fileReader = new FileReader()


fileReader.onload = () => {
   const base64 = fileReader.result.replace(
    /^data:image\/(png|jpeg|jpg);base64,/,''
   )
   console.log(input.files[0]);
   console.log(base64);
}

input.addEventListener('change', () => {
   fileReader.readAsDataURL(input.files[0])
})
}

init();