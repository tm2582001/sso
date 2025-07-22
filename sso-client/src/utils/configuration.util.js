import baseConfig from "../configurations/base.json" with { type: "json" };;

// TODO: improve configuration build

const buildConfiguration = () => {
  let configurations = baseConfig;

  for (const key in process.env) {
    if (key.startsWith("APP_")) {
        let keyLevel1 = "", keyLevel2 = "";
        let configKey = key.replace("APP_", "");

        if(configKey.includes("__")){
            keyLevel1 = configKey.split("__")[0];
            keyLevel2 = configKey.split("__")[1];
        }else{
            keyLevel1 = configKey;
        }

        if(keyLevel2){
            if(!configurations[keyLevel1]){
                configurations[keyLevel1] = {};
            }
            configurations[keyLevel1][keyLevel2] = process.env[key];
        }else{
            configurations[keyLevel1] = process.env[key];
        }
    }
  }

  return configurations;
};

export default buildConfiguration;
