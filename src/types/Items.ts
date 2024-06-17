declare module Items {
    type PluginItem = {
        num: number;
        name: string;
        privilege: number;
        filename:string
    };

    //事件的消息体
    interface Payload {
        message: string;
    }

}