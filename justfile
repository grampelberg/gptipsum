git_version := `git rev-parse --short HEAD 2>/dev/null || echo "unknown"`
registry := "ghcr.io/grampelberg"
image_name := "llmipsum"
tag := "sha-" + git_version
image := registry + "/" + image_name + ":" + tag

build-image:
    docker build -t {{ image }} --secret "id=api-key,env=API_KEY" -f docker/llmipsum.dockerfile .

upload-image:
    docker push {{ image }}

dev-push registry=env_var('LOCAL_REGISTRY'):
    just registry="{{ registry }}" tag="latest" build-image upload-image
