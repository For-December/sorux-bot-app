<template>


  <el-container>
    <el-aside width="1" class="bg-[#545c64] h-[100vh]">
      <el-scrollbar>
        <SidebarMenu/>
      </el-scrollbar>


    </el-aside>
    <el-container>
      <el-header>

        <el-button type="warning" @click="onLogout">登出</el-button>
      </el-header>
      <el-main>
        <router-view/>
      </el-main>
    </el-container>
  </el-container>


</template>

<script lang="ts" setup>


import SidebarMenu from "@/layout/components/SidebarMenu.vue";
import {useUserStore} from "@/store/userStore.ts";

const userStore = useUserStore()
const onLogout = () => {
  userStore.logout()

}

</script>

<style scoped>


</style>
