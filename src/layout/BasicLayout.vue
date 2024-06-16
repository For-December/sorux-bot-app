<template>


  <el-container>
    <el-aside width="1" class="bg-[#545c64] h-[100vh]">
      <el-scrollbar>
        <SidebarMenu/>
      </el-scrollbar>


    </el-aside>
    <el-container>
      <el-header>Header</el-header>
      <el-main>
        <router-view/>
      </el-main>
    </el-container>
  </el-container>


</template>

<script lang="ts" setup>


import SidebarMenu from "@/layout/components/SidebarMenu.vue";
</script>

<style scoped>


</style>
