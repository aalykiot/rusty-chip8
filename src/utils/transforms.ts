import axios from 'axios';

const toArrayBuffer = (file: File): Promise<ArrayBuffer> => {
  return new Promise((resolve) => {
    const reader = new FileReader();
    reader.onload = () => {
      resolve(<ArrayBuffer>reader.result);
    };
    reader.readAsArrayBuffer(file);
  });
};

const toUint8Array = async (file: File): Promise<Uint8Array> => {
  const buffer = await toArrayBuffer(file);
  return new Uint8Array(buffer);
};

const getUint8Array = (url: string): Promise<Uint8Array> => {
  return axios
    .get(url, { responseType: 'arraybuffer' })
    .then((response) => new Blob([response.data]))
    .then((blob) => blob.arrayBuffer())
    .then((buffer) => new Uint8Array(buffer));
};

export { toUint8Array, getUint8Array };
