package models

import (
	"github.com/spf13/viper"
)

const (
	ProviderMoonShot ModelProvider = "moonshot"
)

var MoonShotModels = map[ModelID]Model{
	"moonshot.moonshot-v1-8k": {
		ID:                  "moonshot.moonshot-v1-8k",
		Name:                "MoonShot V1 8K",
		Provider:            ProviderMoonShot,
		APIModel:            "moonshot-v1-8k",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		CostPer1MInCached:   0,
		CostPer1MOutCached:  0,
		ContextWindow:       8192,
		DefaultMaxTokens:    4096,
		CanReason:           true,
		SupportsAttachments: true,
	},
	"moonshot.moonshot-v1-32k": {
		ID:                  "moonshot.moonshot-v1-32k",
		Name:                "MoonShot V1 32K",
		Provider:            ProviderMoonShot,
		APIModel:            "moonshot-v1-32k",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		CostPer1MInCached:   0,
		CostPer1MOutCached:  0,
		ContextWindow:       32768,
		DefaultMaxTokens:    8192,
		CanReason:           true,
		SupportsAttachments: true,
	},
	"moonshot.moonshot-v1-128k": {
		ID:                  "moonshot.moonshot-v1-128k",
		Name:                "MoonShot V1 128K",
		Provider:            ProviderMoonShot,
		APIModel:            "moonshot-v1-128k",
		CostPer1MIn:         0,
		CostPer1MOut:        0,
		CostPer1MInCached:   0,
		CostPer1MOutCached:  0,
		ContextWindow:       131072,
		DefaultMaxTokens:    16384,
		CanReason:           true,
		SupportsAttachments: true,
	},
}

func init() {
	for _, model := range MoonShotModels {
		SupportedModels[model.ID] = model
	}

	ProviderPopularity[ProviderMoonShot] = 10
	viper.SetDefault("providers.moonshot.endpoint", "https://api.moonshot.cn/v1")
	viper.SetDefault("agents.coder.model", "moonshot.moonshot-v1-8k")
	viper.SetDefault("agents.summarizer.model", "moonshot.moonshot-v1-8k")
	viper.SetDefault("agents.task.model", "moonshot.moonshot-v1-8k")
	viper.SetDefault("agents.title.model", "moonshot.moonshot-v1-8k")
}
