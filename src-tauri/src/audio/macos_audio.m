#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

@interface AudioRecorderDelegate : NSObject <AVAudioRecorderDelegate>
@end

@implementation AudioRecorderDelegate
- (void)audioRecorderDidFinishRecording:(AVAudioRecorder *)recorder successfully:(BOOL)flag {
    NSLog(@"Recording finished: %@", flag ? @"YES" : @"NO");
}

- (void)audioRecorderEncodeErrorDidOccur:(AVAudioRecorder *)recorder error:(NSError *)error {
    NSLog(@"Recording error: %@", error);
}
@end

static AVAudioRecorder *recorder = nil;
static AudioRecorderDelegate *delegate = nil;

void start_macos_recording(const char *path) {
    @autoreleasepool {
        NSURL *url = [NSURL fileURLWithPath:[NSString stringWithUTF8String:path]];

        NSDictionary *settings = @{
            AVFormatIDKey: @(kAudioFormatLinearPCM),
            AVSampleRateKey: @44100.0,
            AVNumberOfChannelsKey: @1,
            AVLinearPCMBitDepthKey: @16,
            AVLinearPCMIsBigEndianKey: @NO,
            AVLinearPCMIsFloatKey: @NO,
            AVEncoderAudioQualityKey: @(AVAudioQualityHigh)
        };

        NSError *error = nil;

        if (delegate == nil) {
            delegate = [[AudioRecorderDelegate alloc] init];
        }

        recorder = [[AVAudioRecorder alloc] initWithURL:url settings:settings error:&error];
        if (error) {
            NSLog(@"Error creating recorder: %@", error);
            return;
        }

        recorder.delegate = delegate;
        recorder.meteringEnabled = YES;
        [recorder prepareToRecord];

        BOOL success = [recorder record];
        NSLog(@"Recording started: %@ to %@", success ? @"YES" : @"NO", url.path);

        if (success) {
            // Log mic level periodically
            dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
                while (recorder && recorder.isRecording) {
                    [recorder updateMeters];
                    float level = [recorder averagePowerForChannel:0];
                    NSLog(@"Mic level: %.2f dB", level);
                    [NSThread sleepForTimeInterval:1.0];
                }
            });
        }
    }
}

void stop_macos_recording(void) {
    @autoreleasepool {
        if (recorder && recorder.isRecording) {
            [recorder stop];
            NSLog(@"Recording stopped");
        }
        recorder = nil;
    }
}
