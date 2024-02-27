<template>
    <div class="p-4">
      <div class="flex items-center mb-4">
        <select v-model="sortBy" class="select select-bordered max-w-xs focus:outline-none">
          <option value="ATTENTION">热点排序</option>
          <option value="NEW">最新排序</option>
          <option value="RECOMMEND">推荐排序</option>
        </select>
        <input v-model="searchKeyword" type="text" class="flex-grow p-2 border border-gray-300 focus:outline-none" placeholder="搜索演唱会门票...">
        <button @click="search" class="p-2 bg-blue-500 text-white rounded-r-md">搜索</button>
      </div>
      <div v-if="loading" class="text-center">加载中...</div>
      <div v-else-if="error" class="text-center text-red-500">{{ error }}</div>
      <div v-else>
        <div v-if="concerts.length === 0 && !reachedEnd" class="text-center text-gray-500">暂无演唱会门票</div>
        <div v-else>
          <div v-for="concert in sortedConcerts" :key="concert.id" class="border p-4 mb-4 flex items-start">
            <img :src="concert.posterUrl" alt="Concert Image" class="w-48 h-48 object-cover rounded-md mr-4">
            <div class="flex-grow">
              <h3 class="text-lg font-bold mb-2">{{ concert.showName }}</h3>
              <p><span class="font-bold">日期:</span> {{ concert.showDate }}</p>
              <p><span class="font-bold">城市:</span> {{ concert.cityName }}</p>
              <p><span class="font-bold">场地:</span> {{ concert.venueName }}</p>
              <p v-if="concert.latestSaleTime"><span class="font-bold">开售时间:</span> {{ concert.latestSaleTime }}</p>
              <div class="flex items-center mt-2">
                <p class="text-lg font-bold mr-auto">{{ concert.minOriginalPrice }} 元起</p>
                <div class="flex items-center mt-2">
                  <button v-if="concert.showStatus === 'PENDING' && concert.latestSaleTime" class="p-2 bg-blue-500 text-white rounded-md">加入抢票</button>
                  <button v-else-if="concert.showStatus === 'PENDING'" class="p-2 bg-blue-500 text-white rounded-md">添加提醒</button>
                  <button v-else-if="concert.showStatus === 'ONSALE'" class="p-2 bg-blue-500 text-white rounded-md">立即购买</button>
                  <button v-else-if="concert.showStatus=== 'PRESALE'" class="p-2 bg-blue-500 text-white rounded-md">缺票登记</button>
                </div>
              </div>
            </div>
          </div>
          <div v-if="loadingNextPage" class="text-center">加载中...</div>
          <div v-else-if="!reachedEnd" ref="scrollLoader"></div>
          <div v-else class="text-center text-gray-500">已经没有更多内容了</div>
        </div>
      </div>
    </div>
  </template>
  
<script lang="ts">
import { invoke } from '@tauri-apps/api';
import { defineComponent } from 'vue';
  
  interface Concert {
    id: number;
    title: string;
    image: string;
    date: string;
    city: string;
    venue: string;
    price: number;
  }
  
  export default defineComponent({
    name: 'ConcertList',
    data() {
      return {
        sortBy: 'NEW', // 默认按热点排序
        searchKeyword: '',
        loading: false,
        error: '',
        concerts: [] as Concert[],
        currentPage: 1,
        loadingNextPage: false,
        reachedEnd: false
      };
    },
    computed: {
      sortedConcerts(): Concert[] {
        return this.concerts;
      }
    },
    methods: {
      async search() {
        this.currentPage = 1; // 重置页码
        this.loading = true;
        try {
          await this.fetchConcerts();
        } catch (error) {
          this.error = '加载失败，请重试';
        } finally {
          this.loading = false;
        }
      },
      observeScroll() {
        const scrollObserver = new IntersectionObserver(entries => {
          const entry = entries[0];
          if (entry.isIntersecting && !this.loadingNextPage && !this.reachedEnd) {
            this.loadNextPage();
          }
        });
        scrollObserver.observe(this.$refs.scrollLoader as Element);
      },
      async fetchConcerts() {
        const response: any = await invoke("search_show_list", {keyword: this.searchKeyword, sortType: this.sortBy, page: this.currentPage})
        console.log(response)
        if (response.statusCode != 200) {
          return
        }
        if (response.data.searchData.length == 0) {
            this.reachedEnd = true;
          }else{
            if (this.currentPage === 1) {
              this.concerts = response.data.searchData;
            } else {
              this.concerts = [...this.concerts, ...response.data.searchData];
            }
            this.currentPage++;
        }
      },
      async loadNextPage() {
        if (!this.loadingNextPage && !this.reachedEnd) {
          this.loadingNextPage = true;
          try {
            await this.fetchConcerts();
          } catch (error) {
            console.error('加载下一页失败', error);
          } finally {
            this.loadingNextPage = false;
          }
        }
      }
    },
    mounted() {
      this.$refs.scrollLoader && this.observeScroll();
    },

  });
  </script>
  
  <style scoped>
  /* Add custom styles here */
  </style>
  