import {invoke} from "@tauri-apps/api";
import {listen} from "@tauri-apps/api/event";
import {ref} from "vue";

//事件的消息体
interface Payload {
    message: string;

}

let unListen: any = null

export const qrcodeImageSrc = ref('')
export const startQrcodeListener = () => {
    invoke('watch_qrcode', {})
    //防止重复监听
    if (unListen != null) {
        console.log("already listen");
        return;
    }

    const start_listen = async () => {
        //这里与后端保持一致
        return await listen<Payload>('qrcode-event', (event) => {
            const {message} = event.payload;
            // console.log("message:", message);
            qrcodeImageSrc.value = 'data:image/png;base64,'+message;
        });
    };
    unListen = start_listen();
}


export const stopQrcodeListen = () => {
    console.log("is_listening:", unListen != null);
    if (unListen != null) {
        unListen.then((ok: any) => {
            ok();
            unListen = null;
            console.log("stop success");
        }).catch((err: any) => {
            console.log("stop fail", err);
        })
    }
}
