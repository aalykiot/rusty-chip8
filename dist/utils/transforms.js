import axios from "../../__snowpack/pkg/axios.js";
const toArrayBuffer = (file) => {
  return new Promise((resolve) => {
    const reader = new FileReader();
    reader.onload = () => {
      resolve(reader.result);
    };
    reader.readAsArrayBuffer(file);
  });
};
const toUint8Array = async (file) => {
  const buffer = await toArrayBuffer(file);
  return new Uint8Array(buffer);
};
const getUint8Array = (url) => {
  return axios.get(url, {responseType: "arraybuffer"}).then((response) => new Blob([response.data])).then((blob) => blob.arrayBuffer()).then((buffer) => new Uint8Array(buffer));
};
export {toUint8Array, getUint8Array};
