async function init() {
  let rustApp = null;

  try {
    rustApp = await import('../pkg');
  } catch (error) {
    console.error(error);
  }

  const input = document.getElementById('upload');
  const fileReader = new FileReader();

  fileReader.onloadend = () => {
    let base64File = fileReader.result.replace(/^data:image\/(png|jpeg|jpg);base64,/, '');
    let grayScaledImageURL = rustApp.grayscale(base64File);

    document.getElementById('vintage-image').setAttribute('src', grayScaledImageURL);
  };

  input.addEventListener('change', () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();