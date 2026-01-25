<script setup lang="ts">
import { useConfigStore } from '../stores/config'
import { storeToRefs } from 'pinia'
import { computed, ref } from 'vue'
import PageHeader from '../components/PageHeader.vue'

const store = useConfigStore()
const { currentConfig } = storeToRefs(store)

const services = computed(() => {
    const list: any[] = []
    if (currentConfig.value?.client?.services) {
        Object.entries(currentConfig.value.client.services).forEach(([name, svc]: [string, any]) => {
            list.push({ name, ...svc })
        })
    }
    return list
})

const dialogVisible = ref(false)
const editingService = ref<any>({})
const isNew = ref(false)

function editService(svc: any) {
    editingService.value = { ...svc }
    isNew.value = false
    dialogVisible.value = true
}

function addService() {
    editingService.value = { name: '', local_addr: '127.0.0.1:80', token: '' }
    isNew.value = true
    dialogVisible.value = true
}

function saveService() {
    const cfg = JSON.parse(JSON.stringify(currentConfig.value || {}))
    if (!cfg.client) cfg.client = {}
    if (!cfg.client.services) cfg.client.services = {}
    cfg.client.services[editingService.value.name] = {
        local_addr: editingService.value.local_addr,
        token: editingService.value.token
    }
    store.updateFromObject(cfg)
    dialogVisible.value = false
}

function deleteService(name: string) {
    const cfg = JSON.parse(JSON.stringify(currentConfig.value || {}))
    if (cfg.client?.services?.[name]) {
        delete cfg.client.services[name]
        store.updateFromObject(cfg)
    }
}
</script>

<template>
  <div class="proxy-page">
    <PageHeader>
      <template #header>
        <el-button type="primary" @click="addService">{{ $t('proxy.addService') }}</el-button>
      </template>
    </PageHeader>

    <div class="proxy-content">
      <div class="service-list">
        <el-empty v-if="services.length === 0" :description="$t('proxy.noServices')" />
        <el-row :gutter="20" v-else>
            <el-col :span="12" v-for="svc in services" :key="svc.name" style="margin-bottom: 20px;">
                <el-card shadow="hover">
                    <template #header>
                        <div class="card-header">
                            <span>{{ svc.name }}</span>
                            <el-tag size="small" type="success">{{ $t('proxy.client') }}</el-tag>
                        </div>
                    </template>
                    <div class="svc-details">
                        <p>{{ $t('proxy.address') }}: {{ svc.local_addr }}</p>
                        <p>Token: {{ svc.token ? '********' : '未设置' }}</p>
                    </div>
                    <div class="actions">
                        <el-button size="small" @click="editService(svc)">{{ $t('common.edit') }}</el-button>
                        <el-button size="small" type="danger" @click="deleteService(svc.name)">{{ $t('common.delete') }}</el-button>
                    </div>
                </el-card>
            </el-col>
        </el-row>
      </div>

      <el-dialog v-model="dialogVisible" :title="isNew ? $t('proxy.addService') : $t('proxy.editService')">
        <el-form :model="editingService" label-width="100px">
            <el-form-item :label="$t('proxy.name')">
                <el-input v-model="editingService.name" :disabled="!isNew" />
            </el-form-item>
            <el-form-item :label="$t('proxy.address')">
                <el-input v-model="editingService.local_addr" placeholder="127.0.0.1:80" />
            </el-form-item>
            <el-form-item label="Token">
                <el-input v-model="editingService.token" type="password" show-password placeholder="与服务端一致的令牌" />
            </el-form-item>
        </el-form>
        <template #footer>
            <el-button @click="dialogVisible = false">{{ $t('common.cancel') }}</el-button>
            <el-button type="primary" @click="saveService">{{ $t('common.confirm') }}</el-button>
        </template>
      </el-dialog>
    </div>
  </div>
</template>

<style scoped>
.proxy-page {
    display: flex;
    flex-direction: column;
    height: 100%;
}

.proxy-content {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.actions {
    margin-top: 10px;
    text-align: right;
    display: flex;
    gap: 8px;
    justify-content: flex-end;
}

.service-list {
}
</style>

