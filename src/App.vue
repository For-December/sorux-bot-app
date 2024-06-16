<script setup lang="ts">
import {invoke} from "@tauri-apps/api";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

//事件的消息体
interface Payload {
  message: string;

}

let unListen: any = null
const startListen = () => {
  invoke('init_process', {})
  //防止重复监听
  if (unListen != null) {
    console.log("already listen");
    return;
  }

  const start_listen = async () => {
    //注意这里的my-event名称，要与后端保持一致
    return await listen<Payload>('my-event', (event) => {
      const {message} = event.payload;
      console.log("message:", message);
    });
  };
  unListen = start_listen();
}


const stopListen = () => {
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
startListen()

</script>

<template>
  <router-view/>
</template>

<style scoped>

</style>
