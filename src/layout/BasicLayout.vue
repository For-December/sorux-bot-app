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
import {useRouter} from "vue-router";

const userStore = useUserStore()
const router = useRouter()
const onLogout = () => {
  userStore.logout()
  router.push("/login")
}

</script>

<style scoped>


</style>
