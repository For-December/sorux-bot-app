<script setup lang="ts">
import Terminal from "@/pages/Terminal.vue";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api";
import {listen} from "@tauri-apps/api/event";

import Payload = Items.Payload;






const terminal = ref();

let unListen: any = null;
const startListen = () => {
  invoke("provider_logs", {});
  //防止重复监听
  if (unListen != null) {
    console.log("already listen");
    return;
  }


  const start_listen = async () => {
    //注意这里的my-event名称，要与后端保持一致
    return await listen<Payload>("provider-logs-event", (event) => {
      const { message } = event.payload;
      console.log("message:", message);
      terminal.value.addLine(message);
    });
  };

  unListen = start_listen()
};

startListen()




</script>

<template>
<!--    <button @click="addLogLine">Add Log Line</button>-->
  <Terminal ref="terminal" />

<!--  <el-tree-->
<!--      style="max-width: 600px"-->
<!--      :data="data"-->
<!--      :props="defaultProps"-->
<!--      @node-click="handleNodeClick"-->
<!--  />-->
</template>

<style scoped>

</style>