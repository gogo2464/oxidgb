pipeline {
  agent none
  stages {
    stage('Git') {
      steps {
        parallel(
          "Linux": {
            node ("linux") {
              git url: 'https://github.com/j-selby/Oxidgb.git', credentialsId: "github"
            }
          },
          "Windows": {
            node ("windows") {
              git url: 'https://github.com/j-selby/Oxidgb.git', credentialsId: "github"
            }
          }
        )
      }
    }
    stage('Clean') {
      steps {
        parallel(
          "Linux": {
            node ("linux") {
              dir("build") {
                deleteDir()
              }
              sh 'cargo clean'
            }
          },
          "Windows": {
            node ("windows") {
              dir("build") {
                deleteDir()
              }
              bat 'cargo clean'
            }
          }
        )
      }
    }
    stage('Build') {
      steps {
        parallel(
          "Linux": {
            node ("linux") {
              sh '''
              cargo build --release

              strip target/release/oxidgb_glutin
              
              mkdir build
              cp target/release/oxidgb_glutin build/
              
              cd build
              
              tar czf oxidgb-linux-x64.tar.gz ./*
              '''
              
              stash(name: 'linuxbuild', includes: 'build/oxidgb-linux-x64.tar.gz')
            }
          },
          "Windows": {
            node ("windows") {
              bat '''
              cargo build --release
              
              mkdir build
              copy /Y target\\release\\oxidgb_glutin.exe build\\
              
              cd build
              
              7z a -tzip oxidgb-windows-x64.zip .\\*
              '''
              
              stash(name: 'windowsbuild', includes: 'build/oxidgb-windows-x64.zip')
            }
          }
        )
      }
    }
    stage('Deploy') {
      steps {
        node ("linux") {
          unstash 'linuxbuild'
          unstash 'windowsbuild'
          sh '''
          mv build/oxidgb-windows-x64.zip ..
          mv build/oxidgb-linux-x64.tar.gz ..
          '''
          archiveArtifacts(artifacts: 'oxidgb*', onlyIfSuccessful: true)
        }
      }
    }
  }
}
