<template>


  <el-container>
    <el-aside width="200px">
      <SidebarMenu/>


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
