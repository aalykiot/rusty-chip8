function toArrayBuffer(file: File): Promise<ArrayBuffer> {
  return new Promise(resolve => {
    const reader = new FileReader();
    reader.onload = () => {
      resolve(<ArrayBuffer>reader.result);
    };
    reader.readAsArrayBuffer(file);
  });
}

export async function toUint8Array(file: File): Promise<Uint8Array> {
  const buffer = await toArrayBuffer(file);
  return new Uint8Array(buffer);
}