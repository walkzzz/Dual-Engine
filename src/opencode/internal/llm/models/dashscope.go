package models

import (
	"github.com/spf13/viper"
)

const (
	ProviderDashScope ModelProvider = "dashscope"
)

var DashScopeModels = map[ModelID]Model{
	"dashscope.qwen2.5-coder-32b-instruct": {
		ID:                  "dashscope.qwen2.5-coder-32b-instruct",
		Name:                "Qwen Coder 2.5 32B",
		Provider:            ProviderDashScope,
		APIModel:            "qwen2.5-coder-32b-instruct",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		CostPer1MInCached:   0,
		CostPer1MOutCached:  0,
		ContextWindow:       32768,
		DefaultMaxTokens:    8192,
		CanReason:           true,
		SupportsAttachments: true,
	},
	"dashscope.qwen2.5-coder-14b-instruct": {
		ID:                  "dashscope.qwen2.5-coder-14b-instruct",
		Name:                "Qwen Coder 2.5 14B",
		Provider:            ProviderDashScope,
		APIModel:            "qwen2.5-coder-14b-instruct",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		CostPer1MInCached:   0,
		CostPer1MOutCached:  0,
		ContextWindow:       32768,
		DefaultMaxTokens:    8192,
		CanReason:           true,
		SupportsAttachments: true,
	},
	"dashscope.qwen2.5-coder-7b-instruct": {
		ID:                  "dashscope.qwen2.5-coder-7b-instruct",
		Name:                "Qwen Coder 2.5 7B",
		Provider:            ProviderDashScope,
		APIModel:            "qwen2.5-coder-7b-instruct",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		CostPer1MInCached:   0,
		CostPer1MOutCached:  0,
		ContextWindow:       32768,
		DefaultMaxTokens:    8192,
		CanReason:           true,
		SupportsAttachments: true,
	},
	"dashscope.qwen-plus": {
		ID:                  "dashscope.qwen-plus",
		Name:                "Qwen Plus",
		Provider:            ProviderDashScope,
		APIModel:            "qwen-plus",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		ContextWindow:       32768,
		DefaultMaxTokens:    8192,
		CanReason:           true,
		SupportsAttachments: true,
	},
	"dashscope.qwen-turbo": {
		ID:                  "dashscope.qwen-turbo",
		Name:                "Qwen Turbo",
		Provider:            ProviderDashScope,
		APIModel:            "qwen-turbo",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		ContextWindow:       8192,
		DefaultMaxTokens:    4096,
		CanReason:           true,
		SupportsAttachments: true,
	},
	"dashscope.qwen-max": {
		ID:                  "dashscope.qwen-max",
		Name:                "Qwen Max",
		Provider:            ProviderDashScope,
		APIModel:            "qwen-max",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		ContextWindow:       8192,
		DefaultMaxTokens:    4096,
		CanReason:           true,
		SupportsAttachments: true,
	},
}

func init() {
	for _, model := range DashScopeModels {
		SupportedModels[model.ID] = model
	}

	ProviderPopularity[ProviderDashScope] = 10
	viper.SetDefault("providers.dashscope.endpoint", "https://dashscope.aliyuncs.com/compatible-mode/v1")
	viper.SetDefault("agents.coder.model", "dashscope.qwen2.5-coder-32b-instruct")
	viper.SetDefault("agents.summarizer.model", "dashscope.qwen2.5-coder-32b-instruct")
	viper.SetDefault("agents.task.model", "dashscope.qwen2.5-coder-32b-instruct")
	viper.SetDefault("agents.title.model", "dashscope.qwen2.5-coder-32b-instruct")
}
