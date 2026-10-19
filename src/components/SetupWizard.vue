<template>
  <div class="fixed inset-0 flex items-center justify-center bg-surface-alt">
    <div class="w-[680px] bg-surface rounded-2xl shadow-2xl border border-edge overflow-hidden">
      <!-- 标题 + 操作按钮 -->
      <div class="px-8 pt-8 pb-4 flex items-start justify-between">
        <div>
          <h1 class="text-2xl font-bold text-content">
            {{ props.editProfile ? t('setup.editTitle') : t('setup.title') }}
          </h1>
          <p class="text-sm text-content-secondary mt-1">
            {{ props.editProfile ? t('setup.editSubtitle') : t('setup.subtitle') }}
          </p>
        </div>
        <div class="flex items-center gap-2 flex-shrink-0">
          <!-- 返回配置列表 -->
          <button
            v-if="props.showBack"
            @click="emit('back')"
            class="flex items-center gap-1.5 px-3 py-1.5 text-sm text-content-secondary hover:text-content bg-surface-dim rounded-lg hover:bg-slate-200 transition-colors"
          >
            <ArrowLeft class="w-4 h-4" />
            <span>{{ t('profile.selectTitle') }}</span>
          </button>
          <!-- 语言切换 -->
          <button
            @click="toggleLang"
            class="flex items-center gap-1.5 px-3 py-1.5 text-sm text-content-secondary hover:text-content bg-surface-dim rounded-lg hover:bg-slate-200 transition-colors"
            :title="nextLocaleLabel"
          >
            <Languages class="w-4 h-4" />
            <span>{{ currentLocaleLabel }}</span>
          </button>
        </div>
      </div>

      <!-- 步骤指示器 -->
      <div class="px-8 pb-4">
        <div class="flex items-center gap-2">
          <template v-for="(step, idx) in steps" :key="idx">
            <div
              class="flex items-center gap-1.5 text-sm"
              :class="idx <= currentStep ? 'text-indigo-600 font-medium' : 'text-content-tertiary'"
            >
              <div
                class="w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold"
                :class="
                  idx < currentStep
                    ? 'bg-indigo-600 text-white'
                    : idx === currentStep
                      ? 'bg-indigo-100 text-indigo-600 border-2 border-indigo-600'
                      : 'bg-surface-dim text-content-tertiary'
                "
              >
                <Check v-if="idx < currentStep" class="w-3.5 h-3.5" />
                <span v-else>{{ idx + 1 }}</span>
              </div>
              <span class="hidden sm:inline">{{ step }}</span>
            </div>
            <div
              v-if="idx < steps.length - 1"
              class="flex-1 h-px"
              :class="idx < currentStep ? 'bg-indigo-600' : 'bg-edge'"
            />
          </template>
        </div>
      </div>

      <!-- 步骤内容 -->
      <div class="px-8 py-4 min-h-[340px]">
        <!-- Step 0: 后端大类 -->
        <div v-if="currentStep === 0" class="space-y-3">
          <div
            v-for="opt in backendOptions"
            :key="opt.value"
            @click="selectBackend(opt.value)"
            class="flex items-center gap-4 p-4 rounded-xl border-2 cursor-pointer transition-all"
            :class="
              backendKind === opt.value
                ? 'border-indigo-600 bg-indigo-50/50'
                : 'border-edge hover:border-slate-300'
            "
          >
            <div
              class="w-10 h-10 rounded-lg flex items-center justify-center"
              :class="
                backendKind === opt.value
                  ? 'bg-indigo-100 text-indigo-600'
                  : 'bg-surface-dim text-content-secondary'
              "
            >
              <component :is="opt.icon" class="w-5 h-5" />
            </div>
            <div class="flex-1">
              <div class="font-medium text-content">{{ opt.label }}</div>
              <div class="text-sm text-content-secondary">{{ opt.desc }}</div>
            </div>
            <div
              class="w-5 h-5 rounded-full border-2 flex items-center justify-center"
              :class="backendKind === opt.value ? 'border-indigo-600' : 'border-slate-300'"
            >
              <div
                v-if="backendKind === opt.value"
                class="w-2.5 h-2.5 rounded-full bg-indigo-600"
              />
            </div>
          </div>
        </div>

        <!-- Step 1: 连接配置 -->
        <div v-if="currentStep === 1" class="space-y-4">
          <!-- Profile 名称 -->
          <div>
            <label class="block text-sm font-medium text-content-secondary mb-1">
              {{ t('setup.profileName') }}
            </label>
            <input
              v-model="form.name"
              type="text"
              :placeholder="t('setup.profileNamePlaceholder')"
              class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              @input="userEdited.name = true"
            />
          </div>

          <!-- 数据库引擎切换（仅数据库后端） -->
          <div v-if="!isServerBackend && !isGitBackend">
            <label class="block text-sm font-medium text-content-secondary mb-1">
              {{ t('setup.dbEngine') }}
            </label>
            <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
              <button
                v-for="db in databaseTypeOptions"
                :key="db.value"
                @click="selectDatabaseType(db.value)"
                class="flex-1 px-3 py-1.5 text-sm rounded-md transition-colors"
                :class="
                  form.datasource.type === db.value
                    ? 'bg-surface text-indigo-600 shadow-sm'
                    : 'text-content-secondary hover:text-content'
                "
              >
                {{ db.label }}
              </button>
            </div>
          </div>

          <!-- SQLite 配置 -->
          <template v-if="form.datasource.type === 'sqlite'">
            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.dbPath') }}
              </label>
              <div class="flex gap-2">
                <input
                  v-model="form.datasource.path"
                  type="text"
                  :placeholder="t('setup.dbPathPlaceholder')"
                  class="flex-1 px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  @input="userEdited.path = true"
                />
                <button
                  @click="selectDbPathNew"
                  class="px-3 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors text-sm"
                  :title="t('setup.createNewDb')"
                >
                  {{ t('setup.createNewDb') }}
                </button>
                <button
                  @click="selectDbPathExisting"
                  class="px-3 py-2 bg-surface-dim text-content-secondary rounded-lg hover:bg-slate-200 transition-colors text-sm"
                  :title="t('setup.openExistingDb')"
                >
                  {{ t('setup.openExistingDb') }}
                </button>
              </div>
              <p class="text-xs text-content-tertiary mt-1">{{ t('setup.defaultPath') }}</p>
            </div>
          </template>

          <!-- Git 仓库配置 -->
          <template v-if="form.datasource.type === 'git'">
            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.repoPath') }}
              </label>
              <div class="flex gap-2">
                <input
                  v-model="form.git!.repoPath"
                  type="text"
                  :placeholder="t('setup.repoPathPlaceholder')"
                  class="flex-1 px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  @input="userEdited.repoPath = true"
                />
                <button
                  @click="selectRepoDir"
                  class="px-3 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors text-sm"
                  :title="t('setup.selectRepoDir')"
                >
                  {{ t('setup.selectRepoDir') }}
                </button>
              </div>
              <p class="text-xs text-content-tertiary mt-1">{{ t('setup.repoPathHint') }}</p>
            </div>

            <!-- 远程仓库（可选） -->
            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.gitRemoteUrl') }}
              </label>
              <input
                v-model="form.git!.remoteUrl"
                type="text"
                :placeholder="t('setup.gitRemoteUrlPlaceholder')"
                class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              />
              <p class="text-xs text-content-tertiary mt-1">{{ t('setup.gitRemoteUrlHint') }}</p>
            </div>

            <!-- 远程认证方式（配置了远程地址时显示） -->
            <div v-if="form.git!.remoteUrl?.trim()">
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.gitRemoteAuth') }}
              </label>
              <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
                <button
                  v-for="auth in gitAuthOptions"
                  :key="auth.value"
                  @click="form.git!.remoteAuth = auth.value"
                  class="flex-1 px-3 py-1.5 text-sm rounded-md transition-colors"
                  :class="
                    (form.git!.remoteAuth || 'none') === auth.value
                      ? 'bg-surface text-indigo-600 shadow-sm'
                      : 'text-content-secondary hover:text-content'
                  "
                >
                  {{ auth.label }}
                </button>
              </div>
            </div>

            <!-- HTTPS 用户名 + 密码/PAT -->
            <template v-if="form.git!.remoteUrl?.trim() && form.git!.remoteAuth === 'https'">
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.gitRemoteUsername') }}
                  </label>
                  <input
                    v-model="form.git!.remoteUsername"
                    :placeholder="t('setup.gitRemoteUsernamePlaceholder')"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.gitRemotePassword') }}
                  </label>
                  <input
                    v-model="gitPassword"
                    type="password"
                    :placeholder="t('setup.gitRemotePasswordPlaceholder')"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
              </div>
              <p class="text-xs text-content-tertiary mt-1">
                {{ t('setup.gitRemotePasswordHint') }}
              </p>
            </template>

            <!-- SSH 说明 -->
            <div
              v-if="form.git!.remoteUrl?.trim() && form.git!.remoteAuth === 'ssh'"
              class="p-3 bg-surface-dim rounded-lg text-xs text-content-secondary"
            >
              {{ t('setup.gitRemoteSshHint') }}
            </div>
          </template>

          <!-- MySQL / PostgreSQL 配置 -->
          <template v-if="form.datasource.type === 'mysql' || form.datasource.type === 'postgres'">
            <div class="grid grid-cols-3 gap-3">
              <div class="col-span-2">
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.host') }}
                </label>
                <input
                  v-model="form.datasource.host"
                  :placeholder="t('setup.hostPlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  @input="userEdited.host = true"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.port') }}
                </label>
                <input
                  v-model.number="form.datasource.port"
                  type="number"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.database') }}
              </label>
              <input
                v-model="form.datasource.database"
                :placeholder="t('setup.databasePlaceholder')"
                class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                @input="userEdited.database = true"
              />
            </div>

            <!-- 认证方式 -->
            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.authMethod') }}
              </label>
              <div class="flex gap-1 bg-surface-dim rounded-lg p-1">
                <button
                  @click="form.datasource.authMethod = 'password'"
                  class="flex-1 px-3 py-1.5 text-sm rounded-md transition-colors"
                  :class="
                    form.datasource.authMethod === 'password'
                      ? 'bg-surface text-indigo-600 shadow-sm'
                      : 'text-content-secondary hover:text-content'
                  "
                >
                  {{ t('setup.authPassword') }}
                </button>
                <button
                  @click="form.datasource.authMethod = 'certificate'"
                  class="flex-1 px-3 py-1.5 text-sm rounded-md transition-colors"
                  :class="
                    form.datasource.authMethod === 'certificate'
                      ? 'bg-surface text-indigo-600 shadow-sm'
                      : 'text-content-secondary hover:text-content'
                  "
                >
                  {{ t('setup.authCertificate') }}
                </button>
              </div>
            </div>

            <!-- 密码认证 -->
            <template v-if="form.datasource.authMethod === 'password'">
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.username') }}
                  </label>
                  <input
                    v-model="form.datasource.username"
                    :placeholder="t('setup.usernamePlaceholder')"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                    @input="userEdited.username = true"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.password') }}
                  </label>
                  <input
                    v-model="dbPassword"
                    type="password"
                    :placeholder="t('setup.passwordPlaceholder')"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  />
                </div>
              </div>
            </template>

            <!-- 证书认证 -->
            <template v-if="form.datasource.authMethod === 'certificate'">
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.username') }}
                </label>
                <input
                  v-model="form.datasource.username"
                  :placeholder="t('setup.usernamePlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                  @input="userEdited.username = true"
                />
              </div>

              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.sslMode') }}
                </label>
                <AppSelect
                  v-model="form.datasource.ssl.mode"
                  :options="sslModeSelectOptions"
                  size="md"
                  class="w-full"
                />
              </div>

              <div class="space-y-2">
                <FileInput :label="t('setup.caCert')" v-model="form.datasource.ssl.caCert" />
                <FileInput
                  :label="t('setup.clientCert')"
                  v-model="form.datasource.ssl.clientCert"
                />
                <FileInput :label="t('setup.clientKey')" v-model="form.datasource.ssl.clientKey" />
              </div>
            </template>
          </template>

          <!-- ENote Server 配置 -->
          <template v-if="isServerBackend">
            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.serverUrl') }}
              </label>
              <input
                v-model="form.server!.url"
                type="url"
                :placeholder="t('setup.serverUrlPlaceholder')"
                class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.serverAuthMethod') }}
              </label>
              <select
                v-model="form.server!.authMethod.type"
                class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
              >
                <option value="none">{{ t('setup.serverAuthNone') }}</option>
                <option value="bearer">{{ t('setup.serverAuthBearer') }}</option>
                <option value="basic">{{ t('setup.serverAuthBasic') }}</option>
                <option value="jwt">{{ t('setup.serverAuthJwt') }}</option>
                <option value="custom-header">{{ t('setup.serverAuthCustomHeader') }}</option>
                <option value="oauth2">{{ t('setup.serverAuthOAuth2') }}</option>
              </select>
            </div>

            <!-- Bearer Token -->
            <div v-if="form.server!.authMethod.type === 'bearer'">
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.serverToken') }}
              </label>
              <input
                v-model="serverToken"
                type="password"
                :placeholder="t('setup.serverTokenPlaceholder')"
                class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <!-- Basic Auth -->
            <template v-if="form.server!.authMethod.type === 'basic'">
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.username') }}
                </label>
                <input
                  v-model="(form.server!.authMethod as any).username"
                  type="text"
                  :placeholder="t('setup.usernamePlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.password') }}
                </label>
                <input
                  v-model="serverPassword"
                  type="password"
                  :placeholder="t('setup.passwordPlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
            </template>

            <!-- JWT -->
            <template v-if="form.server!.authMethod.type === 'jwt'">
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.username') }}
                </label>
                <input
                  v-model="(form.server!.authMethod as any).username"
                  type="text"
                  :placeholder="t('setup.usernamePlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.password') }}
                </label>
                <input
                  v-model="serverPassword"
                  type="password"
                  :placeholder="t('setup.passwordPlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.serverRefreshUrl') }}
                </label>
                <input
                  v-model="(form.server!.authMethod as any).refreshUrl"
                  type="text"
                  :placeholder="t('setup.serverRefreshUrlPlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
            </template>

            <!-- Custom Header -->
            <template v-if="form.server!.authMethod.type === 'custom-header'">
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.serverHeaderName') }}
                </label>
                <input
                  v-model="(form.server!.authMethod as any).headerName"
                  type="text"
                  :placeholder="t('setup.serverHeaderNamePlaceholder')"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.serverHeaderValue') }}
                </label>
                <input
                  v-model="serverHeaderValue"
                  type="password"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
            </template>

            <!-- OAuth 2.0 -->
            <template v-if="form.server!.authMethod.type === 'oauth2'">
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.serverOAuthTokenUrl') }}
                  </label>
                  <input
                    v-model="(form.server!.authMethod as any).tokenUrl"
                    type="text"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-content-secondary mb-1">
                    {{ t('setup.serverOAuthClientId') }}
                  </label>
                  <input
                    v-model="(form.server!.authMethod as any).clientId"
                    type="text"
                    class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                  />
                </div>
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.serverOAuthClientSecret') }}
                </label>
                <input
                  v-model="serverClientSecret"
                  type="password"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-content-secondary mb-1">
                  {{ t('setup.serverOAuthScopes') }}
                </label>
                <input
                  v-model="(form.server!.authMethod as any).scopes"
                  type="text"
                  placeholder="notes.read notes.write"
                  class="w-full px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
            </template>

            <div>
              <label class="block text-sm font-medium text-content-secondary mb-1">
                {{ t('setup.serverTimeout') }}
              </label>
              <input
                v-model.number="form.server!.timeout"
                type="number"
                min="5"
                max="300"
                class="w-24 px-3 py-2 border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>
          </template>
        </div>

        <!-- Step 2: 安全设置 -->
        <div v-if="currentStep === 2" class="space-y-6">
          <!-- Server 后端提示 -->
          <div v-if="isServerBackend" class="p-4 bg-blue-50 border border-blue-200 rounded-xl">
            <p class="text-sm text-blue-700">
              <Globe class="w-4 h-4 inline mr-1" />
              {{ t('setup.serverEncryptionNote') }}
            </p>
          </div>
          <div
            v-if="!isServerBackend"
            class="p-5 rounded-xl border-2 transition-all"
            :class="
              form.security.contentEncryption ? 'border-indigo-600 bg-indigo-50/50' : 'border-edge'
            "
          >
            <div class="flex items-center justify-between mb-2">
              <div class="flex items-center gap-2">
                <Shield class="w-5 h-5 text-indigo-600" />
                <span class="font-medium text-content">{{ t('setup.contentEncryption') }}</span>
              </div>
              <button
                @click="toggleEncryption"
                class="relative w-11 h-6 rounded-full transition-colors"
                :class="form.security.contentEncryption ? 'bg-indigo-600' : 'bg-slate-300'"
              >
                <div
                  class="absolute top-0.5 w-5 h-5 bg-white rounded-full shadow-sm transition-transform"
                  :class="form.security.contentEncryption ? 'translate-x-5.5' : 'translate-x-0.5'"
                />
              </button>
            </div>
            <p class="text-sm text-content-secondary">{{ t('setup.contentEncryptionDesc') }}</p>

            <div v-if="form.security.contentEncryption" class="mt-4 space-y-3">
              <div
                class="flex items-center gap-2 p-3 bg-amber-50 text-amber-700 rounded-lg text-sm"
              >
                <AlertTriangle class="w-4 h-4 flex-shrink-0" />
                <span>{{ t('setup.contentEncryptionWarning') }}</span>
              </div>

              <div
                v-if="encryptionKeyGenerated"
                class="flex items-center gap-2 p-3 bg-green-50 text-green-700 rounded-lg text-sm"
              >
                <Check class="w-4 h-4 flex-shrink-0" />
                <span>{{ t('setup.keyGenerated') }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: 完成 -->
        <div v-if="currentStep === 3" class="flex flex-col items-center justify-center py-8">
          <div v-if="saving" class="flex flex-col items-center gap-4">
            <Loader2 class="w-8 h-8 text-indigo-600 animate-spin" />
            <p class="text-content-secondary">{{ t('setup.saving') }}</p>
          </div>
          <div v-else-if="saveError" class="flex flex-col items-center gap-4 text-center">
            <div class="w-12 h-12 bg-red-100 rounded-full flex items-center justify-center">
              <X class="w-6 h-6 text-red-600" />
            </div>
            <p class="text-red-600">{{ saveError }}</p>
            <button
              @click="currentStep = 1"
              class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors"
            >
              {{ t('setup.prev') }}
            </button>
          </div>
          <div v-else class="flex flex-col items-center gap-4">
            <div class="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center">
              <Check class="w-6 h-6 text-green-600" />
            </div>
            <p class="text-content font-medium">{{ t('setup.testSuccess') }}</p>
          </div>
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="px-8 py-4 border-t border-edge flex justify-between">
        <div class="flex items-center gap-2">
          <button
            v-if="currentStep > 0 && currentStep < 3"
            @click="currentStep--"
            class="px-4 py-2 text-content-secondary hover:text-content transition-colors"
          >
            {{ t('setup.prev') }}
          </button>
          <button
            v-if="currentStep === 1 && !props.editProfile"
            @click="resetToDefaults"
            class="flex items-center gap-1.5 px-3 py-2 text-xs text-content-tertiary hover:text-indigo-600 hover:bg-indigo-50 rounded-lg transition-colors"
            :title="t('setup.resetDefaults')"
          >
            <RotateCcw class="w-3.5 h-3.5" />
            {{ t('setup.resetDefaults') }}
          </button>
        </div>

        <div class="flex gap-2">
          <button
            v-if="
              currentStep === 1 &&
              ((form.datasource.type !== 'sqlite' && !isGitBackend) ||
                (isGitBackend && !!form.git?.remoteUrl?.trim()))
            "
            @click="testConn"
            :disabled="testing"
            class="px-4 py-2 border border-edge text-content-secondary rounded-lg hover:bg-surface-dim transition-colors disabled:opacity-50"
          >
            {{ testing ? t('profile.connecting') : t('setup.testConnection') }}
          </button>
          <button
            v-if="currentStep < 2"
            @click="nextStep"
            :disabled="!canNext"
            class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
          >
            {{ t('setup.next') }}
          </button>
          <button
            v-if="currentStep === 2"
            @click="saveAndConnect"
            :disabled="saving"
            class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors disabled:opacity-50"
          >
            {{ t('setup.saveAndConnect') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { useI18n } from 'vue-i18n'
import { open, save } from '@tauri-apps/plugin-dialog'
import { appDataDir, join } from '@tauri-apps/api/path'
import { profileApi } from '../api/note'
import { showNotification } from './ui/notification'
import { AppSelect } from './ui'
import type { AppSelectOption } from './ui'
import { parseError, parseErrorToAppError } from '../utils/errorHandler'
import { availableLocales, setLocale, getCurrentLocale } from '../i18n'
import type { LocaleType } from '../i18n'
import type { GitRemoteAuth, ProfileConfig } from '../types'
import {
  ArrowLeft,
  Check,
  Shield,
  AlertTriangle,
  X,
  Loader2,
  HardDrive,
  Database,
  Server,
  Globe,
  GitBranch,
  Languages,
  RotateCcw,
} from '@lucide/vue'

const emit = defineEmits<{
  (e: 'complete'): void
  (e: 'back'): void
}>()

const props = defineProps<{
  editProfile?: ProfileConfig & { id?: string }
  /** 是否显示返回按钮（当已有 profile 时显示） */
  showBack?: boolean
}>()

const { t } = useI18n()

const currentLocale = ref<LocaleType>(getCurrentLocale())
const currentLocaleLabel = computed(
  () => availableLocales.find((l) => l.value === currentLocale.value)?.label ?? '',
)
const nextLocaleLabel = computed(() => {
  const next = availableLocales.find((l) => l.value !== currentLocale.value)
  return next?.label ?? ''
})
const toggleLang = () => {
  const next = currentLocale.value === 'zh-CN' ? 'en-US' : 'zh-CN'
  currentLocale.value = next as LocaleType
  setLocale(next as LocaleType)
}

const currentStep = ref(0)
const testing = ref(false)
const saving = ref(false)
const saveError = ref('')
const encryptionKeyGenerated = ref(false)
const encryptionKey = ref('')
const dbPassword = ref('')
// Git HTTPS 认证表单（密码/PAT 独立持有，随保存写入 Keychain，不进配置文件）
const gitPassword = ref('')
// Server 认证表单
const serverToken = ref('')
const serverPassword = ref('')
const serverClientSecret = ref('')
const serverHeaderValue = ref('')

const steps = computed(() => [
  t('setup.step.dbType'),
  t('setup.step.connection'),
  t('setup.step.security'),
  t('setup.step.finish'),
])

const isServerBackend = computed(() => form.datasource.type === 'enote-server')
const isGitBackend = computed(() => form.datasource.type === 'git')

/** 后端大类（与后端 backend 字段对应：database / git / server） */
type BackendKind = 'database' | 'git' | 'server'

/** 从编辑配置推断初始后端大类（兼容缺少 backend 字段的旧配置，按 datasource.type 兜底） */
const inferBackendKind = (): BackendKind => {
  const dbType = props.editProfile?.datasource?.type || 'sqlite'
  if (props.editProfile?.backend === 'server' || dbType === 'enote-server') return 'server'
  if (props.editProfile?.backend === 'git' || dbType === 'git') return 'git'
  return 'database'
}

/** 当前选中的后端大类（Step 0 卡片） */
const backendKind = ref<BackendKind>(inferBackendKind())

const form = reactive<ProfileConfig>({
  name: props.editProfile?.name || '',
  icon: props.editProfile?.icon || '',
  // 兼容缺少 backend 字段的旧配置：按 datasource.type 推断兜底
  backend: props.editProfile?.backend || inferBackendKind(),
  datasource: {
    type: props.editProfile?.datasource?.type || 'sqlite',
    path: props.editProfile?.datasource?.path || '',
    host: props.editProfile?.datasource?.host || 'localhost',
    port: props.editProfile?.datasource?.port || 3306,
    database: props.editProfile?.datasource?.database || '',
    username: props.editProfile?.datasource?.username || '',
    authMethod: props.editProfile?.datasource?.authMethod || 'password',
    ssl: {
      mode: props.editProfile?.datasource?.ssl?.mode || 'disabled',
      caCert: props.editProfile?.datasource?.ssl?.caCert || '',
      clientCert: props.editProfile?.datasource?.ssl?.clientCert || '',
      clientKey: props.editProfile?.datasource?.ssl?.clientKey || '',
    },
  },
  server: props.editProfile?.server || {
    url: '',
    authMethod: { type: 'none' },
    timeout: 30,
  },
  git: {
    repoPath: props.editProfile?.git?.repoPath || '',
    remoteUrl: props.editProfile?.git?.remoteUrl || '',
    remoteAuth: props.editProfile?.git?.remoteAuth || 'none',
    remoteUsername: props.editProfile?.git?.remoteUsername || '',
  },
  security: {
    contentEncryption: props.editProfile?.security?.contentEncryption || false,
  },
})

/** Step 0：后端大类选项（与后端 backend 字段一一对应） */
const backendOptions = computed(() => [
  {
    value: 'database' as BackendKind,
    label: t('setup.dbType.database'),
    desc: t('setup.dbType.databaseDesc'),
    icon: Database,
  },
  {
    value: 'git' as BackendKind,
    label: t('setup.dbType.git'),
    desc: t('setup.dbType.gitDesc'),
    icon: GitBranch,
  },
  {
    value: 'server' as BackendKind,
    label: t('setup.dbType.enoteServer'),
    desc: t('setup.dbType.enoteServerDesc'),
    icon: Globe,
  },
])

/** Step 1：数据库引擎分段选项（仅数据库后端显示） */
const databaseTypeOptions = computed(() => [
  { value: 'sqlite', label: t('setup.dbType.sqlite'), icon: HardDrive },
  { value: 'mysql', label: t('setup.dbType.mysql'), icon: Database },
  { value: 'postgres', label: t('setup.dbType.postgres'), icon: Server },
])

/** Git 远程认证分段选项 */
const gitAuthOptions = computed<{ value: GitRemoteAuth; label: string }[]>(() => [
  { value: 'none', label: t('setup.gitAuthNone') },
  { value: 'https', label: t('setup.gitAuthHttps') },
  { value: 'ssh', label: t('setup.gitAuthSsh') },
])

const sslModeOptions = computed(() => {
  const base = [
    { value: 'disabled', label: t('setup.sslModeDisabled') },
    { value: 'preferred', label: t('setup.sslModePreferred') },
    { value: 'required', label: t('setup.sslModeRequired') },
    { value: 'verify-ca', label: t('setup.sslModeVerifyCa') },
  ]
  if (form.datasource.type === 'mysql') {
    base.push({ value: 'verify-identity', label: t('setup.sslModeVerifyIdentity') })
  } else {
    base.push({ value: 'verify-full', label: t('setup.sslModeVerifyFull') })
  }
  return base
})

const sslModeSelectOptions = computed<AppSelectOption[]>(() => sslModeOptions.value)

const canNext = computed(() => {
  if (currentStep.value === 0) return true
  if (currentStep.value === 1) {
    if (!form.name.trim()) return false
    if (isServerBackend.value) return !!form.server?.url?.trim()
    if (isGitBackend.value) return !!form.git?.repoPath?.trim()
    if (form.datasource.type === 'sqlite') return true
    return !!(form.datasource.host && form.datasource.port && form.datasource.database)
  }
  return true
})

/** 各数据库类型的推荐默认值 */
const dbTypeDefaults: Record<
  string,
  {
    name: string
    host: string
    port: number
    database: string
    username: string
  }
> = {
  sqlite: {
    name: t('setup.defaultName.sqlite'),
    host: '',
    port: 0,
    database: '',
    username: '',
  },
  git: {
    name: t('setup.defaultName.git'),
    host: '',
    port: 0,
    database: '',
    username: '',
  },
  mysql: {
    name: t('setup.defaultName.mysql'),
    host: 'localhost',
    port: 3306,
    database: 'enote',
    username: 'root',
  },
  postgres: {
    name: t('setup.defaultName.postgres'),
    host: 'localhost',
    port: 5432,
    database: 'enote',
    username: 'postgres',
  },
  'enote-server': {
    name: 'ENote Server',
    host: '',
    port: 0,
    database: '',
    username: '',
  },
}

/** 标记用户是否手动修改过某个字段（修改过则不覆盖） */
const userEdited = reactive<Record<string, boolean>>({
  name: !!props.editProfile?.name,
  path: !!props.editProfile?.datasource?.path,
  repoPath: !!props.editProfile?.git?.repoPath,
  host: !!props.editProfile?.datasource?.host,
  database: !!props.editProfile?.datasource?.database,
  username: !!props.editProfile?.datasource?.username,
})

/** 按当前 datasource.type 填充推荐默认值（用户手动修改过的字段不覆盖） */
const applyTypeDefaults = async () => {
  const defaults = dbTypeDefaults[form.datasource.type]
  if (!defaults) return

  // 仅在用户未手动修改时填充默认名称
  if (!userEdited.name || !form.name.trim()) {
    form.name = defaults.name
    userEdited.name = false
  }
  if (form.datasource.type === 'sqlite') {
    // SQLite 默认路径：应用数据目录/enote.db
    if (!userEdited.path || !form.datasource.path) {
      try {
        const dataDir = await appDataDir()
        form.datasource.path = await join(dataDir, 'enote.db')
      } catch {
        // 获取失败时不填充，用户手动选择
      }
    }
  } else if (isGitBackend.value) {
    // Git 默认仓库路径：应用数据目录/notes-repo
    if ((!userEdited.repoPath || !form.git?.repoPath) && form.git) {
      try {
        const dataDir = await appDataDir()
        form.git.repoPath = await join(dataDir, 'notes-repo')
      } catch {
        // 获取失败时不填充，用户手动选择
      }
    }
  } else {
    if (!userEdited.host || !form.datasource.host) {
      form.datasource.host = defaults.host
    }
    form.datasource.port = defaults.port
    if (!userEdited.database || !form.datasource.database) {
      form.datasource.database = defaults.database
    }
    if (!userEdited.username || !form.datasource.username) {
      form.datasource.username = defaults.username
    }
  }
}

/** Step 0 选择后端大类，同步 backend 字段与 datasource.type */
const selectBackend = (kind: BackendKind) => {
  backendKind.value = kind
  form.backend = kind
  if (kind === 'database') {
    if (!['sqlite', 'mysql', 'postgres'].includes(form.datasource.type)) {
      form.datasource.type = 'sqlite'
    }
  } else if (kind === 'git') {
    form.datasource.type = 'git'
  } else {
    form.datasource.type = 'enote-server'
  }
}

/** Step 1 切换数据库引擎，并立即套用该引擎的推荐默认值 */
const selectDatabaseType = async (type: string) => {
  if (form.datasource.type === type) return
  form.datasource.type = type
  await applyTypeDefaults()
}

const nextStep = async () => {
  if (currentStep.value === 0) {
    await applyTypeDefaults()
  }
  currentStep.value++
}

/** 重置为默认值（强制覆盖所有字段） */
const resetToDefaults = async () => {
  const defaults = dbTypeDefaults[form.datasource.type]
  if (!defaults) return

  form.name = defaults.name
  if (form.datasource.type === 'sqlite') {
    try {
      const dataDir = await appDataDir()
      form.datasource.path = await join(dataDir, 'enote.db')
    } catch {
      form.datasource.path = ''
    }
  } else if (isGitBackend.value) {
    try {
      const dataDir = await appDataDir()
      if (form.git) form.git.repoPath = await join(dataDir, 'notes-repo')
    } catch {
      if (form.git) form.git.repoPath = ''
    }
  } else {
    form.datasource.host = defaults.host
    form.datasource.port = defaults.port
    form.datasource.database = defaults.database
    form.datasource.username = defaults.username
    form.datasource.authMethod = 'password'
    form.datasource.ssl.mode = 'disabled'
    form.datasource.ssl.caCert = ''
    form.datasource.ssl.clientCert = ''
    form.datasource.ssl.clientKey = ''
  }
  dbPassword.value = ''

  // 重置编辑标记
  userEdited.name = false
  userEdited.path = false
  userEdited.repoPath = false
  userEdited.host = false
  userEdited.database = false
  userEdited.username = false
}

/** 新建数据库文件（save 对话框，选择保存位置） */
const selectDbPathNew = async () => {
  const path = await save({
    title: t('setup.createNewDb'),
    defaultPath: 'enote.db',
    filters: [{ name: 'SQLite', extensions: ['db'] }],
  })
  if (path) {
    form.datasource.path = path as string
    userEdited.path = true
  }
}

/** 打开已有数据库文件（open 对话框） */
const selectDbPathExisting = async () => {
  const path = await open({
    title: t('setup.openExistingDb'),
    filters: [{ name: 'SQLite', extensions: ['db', 'sqlite', 'sqlite3'] }],
  })
  if (path) {
    form.datasource.path = path as string
    userEdited.path = true
  }
}

/** 选择 Git 仓库目录（open 对话框，目录模式） */
const selectRepoDir = async () => {
  const path = await open({
    title: t('setup.selectRepoDir'),
    directory: true,
  })
  if (path && form.git) {
    form.git.repoPath = path as string
    userEdited.repoPath = true
  }
}

const toggleEncryption = async () => {
  form.security.contentEncryption = !form.security.contentEncryption
  if (form.security.contentEncryption && !encryptionKeyGenerated.value) {
    try {
      encryptionKey.value = await profileApi.generateEncryptionKey()
      encryptionKeyGenerated.value = true
    } catch {
      form.security.contentEncryption = false
    }
  }
}

const testConn = async () => {
  testing.value = true
  try {
    if (isServerBackend.value && form.server) {
      await profileApi.testServerConnection(
        form.server,
        serverToken.value || undefined,
        serverPassword.value || undefined,
        serverHeaderValue.value || undefined,
      )
    } else if (isGitBackend.value) {
      if (!form.git?.remoteUrl?.trim()) {
        throw new Error(t('setup.gitRemoteUrlRequired'))
      }
      await profileApi.testGitConnection(
        form.git.remoteUrl.trim(),
        form.git.remoteAuth || 'none',
        form.git.remoteUsername || undefined,
        gitPassword.value || undefined,
      )
    } else {
      await profileApi.testConnection(form, dbPassword.value || undefined)
    }
    showNotification({ type: 'success', message: t('setup.testSuccess') })
  } catch (e: unknown) {
    const appError = parseErrorToAppError(e)
    showNotification({
      type: 'error',
      message: `${t('setup.testFailed')}: ${appError.message}`,
      detail: appError.details,
    })
  } finally {
    testing.value = false
  }
}

const saveAndConnect = async () => {
  saving.value = true
  saveError.value = ''
  currentStep.value = 3

  try {
    const profileId = props.editProfile?.id || nameToId(form.name) || `profile-${Date.now()}`

    // 先保存 profile 配置到磁盘
    await profileApi.saveProfile(
      profileId,
      form,
      dbPassword.value || undefined,
      form.security.contentEncryption ? encryptionKey.value : undefined,
      // Server 认证信息
      serverToken.value || undefined,
      serverPassword.value || undefined,
      serverClientSecret.value || undefined,
      // Git HTTPS 密码/访问令牌（写入 Keychain）
      gitPassword.value || undefined,
    )

    // 热连接新 profile——后端已改为锁外 fire-and-forget 做远程同步，
    // 连接返回即表示状态切换完成（同步在后台跑，事件通知）。
    // 即使连接阶段有问题也不阻塞进入主界面，用户可手动重试。
    profileApi.reconnectProfile(profileId).catch((e) => {
      saveError.value = parseError(e)
      saving.value = false
    })

    emit('complete')
  } catch (e: unknown) {
    saveError.value = parseError(e)
    saving.value = false
  }
}

const nameToId = (name: string) =>
  name
    .toLowerCase()
    .replace(/[^a-z0-9-_]/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '')

// 文件输入辅助组件（内联）
const FileInput = {
  props: ['label', 'modelValue'],
  emits: ['update:modelValue'],
  template: `
    <div class="flex items-center gap-2">
      <label class="text-sm text-content-secondary w-24 flex-shrink-0">{{ label }}</label>
      <input
        :value="modelValue"
        @input="$emit('update:modelValue', $event.target.value)"
        class="flex-1 px-3 py-1.5 text-sm border border-edge rounded-lg bg-surface text-content focus:outline-none focus:ring-2 focus:ring-indigo-500"
        placeholder="..."
      />
      <button
        @click="selectFile"
        class="px-2 py-1.5 text-xs bg-surface-dim text-content-secondary rounded hover:bg-slate-200 transition-colors"
      >
        ...
      </button>
    </div>
  `,
  setup(
    props: { label: string; modelValue: string },
    { emit }: { emit: (event: string, ...args: unknown[]) => void },
  ) {
    const selectFile = async () => {
      const path = await open({
        title: props.label,
        filters: [{ name: 'Certificate', extensions: ['pem', 'crt', 'key'] }],
      })
      if (path) {
        emit('update:modelValue', path)
      }
    }
    return { selectFile }
  },
}
</script>
